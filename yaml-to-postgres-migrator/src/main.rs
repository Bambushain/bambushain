use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, NotSet, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;

use sheef_dbal::get_database_connection;
use sheef_migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    stderrlog::new()
        .verbosity(log::Level::Info)
        .init()
        .unwrap();

    println!("Create database");
    println!("DATABASE_URL: {}", std::env::var("DATABASE_URL").expect("Needs DATABASE_URL"));

    let db = match get_database_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{err}")
    };

    match Migrator::up(db.into_schema_manager_connection(), None).await {
        Ok(_) => println!("Successfully migrated database"),
        Err(err) => panic!("{err}")
    }

    if let Ok(Some(_)) = sheef_entities::status::Entity::find()
        .filter(sheef_entities::status::Column::Key.eq("YAML_MIGRATED"))
        .one(&db)
        .await {
        println!("Already migrated");
        return;
    }

    let txn = match db.begin().await {
        Ok(txn) => txn,
        Err(err) => panic!("{err}")
    };

    println!("Start migration of yaml database");
    println!("Migrate users");
    let users = match migrate_users(&txn).await {
        Ok(users) => users,
        Err(err) => panic!("{err}")
    };

    println!("Migrate crafters");
    if let Err(err) = migrate_crafters(&txn, users.clone()).await {
        panic!("{err}");
    }

    println!("Migrate fighters");
    if let Err(err) = migrate_fighters(&txn, users.clone()).await {
        panic!("{err}");
    }

    println!("Migrate kills");
    if let Err(err) = migrate_kills(&txn, users.clone()).await {
        panic!("{err}");
    }

    println!("Migrate mounts");
    if let Err(err) = migrate_mounts(&txn, users.clone()).await {
        panic!("{err}");
    }

    println!("Migrate savage mounts");
    if let Err(err) = migrate_savage_mounts(&txn, users.clone()).await {
        panic!("{err}");
    }

    println!("Migrate calendar");
    if let Err(err) = migrate_calendar(&txn, users.clone()).await {
        panic!("{err}");
    }

    println!("Mark database as migrated from yaml to postgres");
    let migrated_status = sheef_entities::status::ActiveModel {
        id: NotSet,
        key: Set("YAML_MIGRATED".to_string()),
        value: Set(true.to_string()),
    };
    if let Err(err) = migrated_status.insert(&txn).await {
        println!("Failed to mark database migrated");
        panic!("{err}");
    }

    println!("Commit transaction");
    if let Err(err) = txn.commit().await {
        panic!("{err}");
    }

    println!("Migration from yaml to postgres done");
}

async fn migrate_users(db: &DatabaseTransaction) -> anyhow::Result<Vec<sheef_entities::user::Model>> {
    let users = sheef_yaml_database::user::get_users(true).await;

    if let Ok(users) = users {
        let users_to_migrate = users.iter().map(|user| sheef_entities::user::ActiveModel {
            id: NotSet,
            username: Set(user.username.clone()),
            password: Set(user.password.clone()),
            is_mod: Set(user.is_mod),
            is_main_group: Set(user.is_main_group),
            gear_level: Set(user.gear_level.clone()),
            job: Set(user.job.clone()),
            is_hidden: Set(user.is_hidden),
        }).collect::<Vec<sheef_entities::user::ActiveModel>>();

        match sheef_entities::user::Entity::insert_many(users_to_migrate).exec_without_returning(db).await {
            Ok(res) => println!("Migrated {res} crafters"),
            Err(err) => return Err(anyhow!(err))
        }

        sheef_entities::user::Entity::find().all(db).await
            .map_err(|err| anyhow!(err))
    } else {
        Err(anyhow!("Failed to migrate users"))
    }
}

async fn migrate_crafters(db: &DatabaseTransaction, users: Vec<sheef_entities::user::Model>) -> anyhow::Result<()> {
    let mut crafters_to_migrate = vec![];

    for user in users {
        if let Ok(crafters) = sheef_yaml_database::crafter::get_crafters(&user.username.clone()).await {
            crafters_to_migrate.append(crafters.iter().map(|crafter| sheef_entities::crafter::ActiveModel {
                id: NotSet,
                job: Set(crafter.job.clone()),
                level: Set(Some(crafter.level.clone())),
                user_id: Set(user.id),
            }).collect::<Vec<sheef_entities::crafter::ActiveModel>>().as_mut());
        }
    }

    match sheef_entities::crafter::Entity::insert_many(crafters_to_migrate).exec_without_returning(db).await {
        Ok(res) => {
            println!("Migrated {res} crafters");
            Ok(())
        }
        Err(err) => Err(anyhow!(err))
    }
}

async fn migrate_fighters(db: &DatabaseTransaction, users: Vec<sheef_entities::user::Model>) -> anyhow::Result<()> {
    let mut fighters_to_migrate = vec![];

    for user in users {
        if let Ok(fighters) = sheef_yaml_database::fighter::get_fighters(&user.username.clone()).await {
            fighters_to_migrate.append(fighters.iter().map(|fighter| sheef_entities::fighter::ActiveModel {
                id: NotSet,
                job: Set(fighter.job.clone()),
                level: Set(Some(fighter.level.clone())),
                gear_score: Set(Some(fighter.gear_score.clone())),
                user_id: Set(user.id),
            }).collect::<Vec<sheef_entities::fighter::ActiveModel>>().as_mut());
        }
    }

    match sheef_entities::fighter::Entity::insert_many(fighters_to_migrate).exec_without_returning(db).await {
        Ok(res) => {
            println!("Migrated {res} fighters");
            Ok(())
        }
        Err(err) => Err(anyhow!(err))
    }
}

async fn migrate_kills(db: &DatabaseTransaction, users: Vec<sheef_entities::user::Model>) -> anyhow::Result<()> {
    let kills_to_migrate = match sheef_yaml_database::kill::get_kills().await {
        Ok(kills) => kills.iter().map(|kill| sheef_entities::kill::ActiveModel {
            id: NotSet,
            name: Set(kill.clone()),
        }).collect::<Vec<sheef_entities::kill::ActiveModel>>(),
        Err(err) => return Err(anyhow!(err)),
    };

    match sheef_entities::kill::Entity::insert_many(kills_to_migrate).exec_without_returning(db).await {
        Ok(res) => println!("Migrated {res} kills"),
        Err(err) => return Err(anyhow!(err))
    }

    let migrated_kills = match sheef_entities::kill::Entity::find().all(db).await {
        Ok(kills) => kills,
        Err(err) => return Err(anyhow!(err))
    };

    let mut kills_to_users = vec![];
    for migrated_kill in migrated_kills {
        let users_for_kill = match sheef_yaml_database::kill::get_users_for_kill(&migrated_kill.name.clone()).await {
            Ok(users) => users,
            Err(err) => return Err(anyhow!(err)),
        };

        for user in users_for_kill {
            if let Some(u) = users.iter().find(|u| u.username == user) {
                kills_to_users.push(sheef_entities::kill_to_user::ActiveModel {
                    user_id: Set(u.id),
                    kill_id: Set(migrated_kill.id),
                });
            }
        }
    }

    match sheef_entities::kill_to_user::Entity::insert_many(kills_to_users).exec_without_returning(db).await {
        Ok(res) => {
            println!("Assigned {res} kills");
            Ok(())
        }
        Err(err) => Err(anyhow!(err))
    }
}

async fn migrate_mounts(db: &DatabaseTransaction, users: Vec<sheef_entities::user::Model>) -> anyhow::Result<()> {
    let mounts_to_migrate = match sheef_yaml_database::mount::get_mounts().await {
        Ok(mounts) => mounts.iter().map(|mount| sheef_entities::mount::ActiveModel {
            id: NotSet,
            name: Set(mount.clone()),
        }).collect::<Vec<sheef_entities::mount::ActiveModel>>(),
        Err(err) => return Err(anyhow!(err)),
    };

    match sheef_entities::mount::Entity::insert_many(mounts_to_migrate).exec_without_returning(db).await {
        Ok(res) => println!("Migrated {res} mounts"),
        Err(err) => return Err(anyhow!(err))
    }

    let migrated_mounts = match sheef_entities::mount::Entity::find().all(db).await {
        Ok(mounts) => mounts,
        Err(err) => return Err(anyhow!(err))
    };

    let mut mounts_to_users = vec![];
    for migrated_mount in migrated_mounts {
        let users_for_mount = match sheef_yaml_database::mount::get_users_for_mount(&migrated_mount.name.clone()).await {
            Ok(users) => users,
            Err(err) => return Err(anyhow!(err)),
        };

        for user in users_for_mount {
            if let Some(u) = users.iter().find(|u| u.username == user) {
                mounts_to_users.push(sheef_entities::mount_to_user::ActiveModel {
                    user_id: Set(u.id),
                    mount_id: Set(migrated_mount.id),
                });
            }
        }
    }

    match sheef_entities::mount_to_user::Entity::insert_many(mounts_to_users).exec_without_returning(db).await {
        Ok(res) => {
            println!("Assigned {res} mounts");
            Ok(())
        }
        Err(err) => Err(anyhow!(err))
    }
}

async fn migrate_savage_mounts(db: &DatabaseTransaction, users: Vec<sheef_entities::user::Model>) -> anyhow::Result<()> {
    let savage_mounts_to_migrate = match sheef_yaml_database::savage_mount::get_savage_mounts().await {
        Ok(savage_mounts) => savage_mounts.iter().map(|savage_mount| sheef_entities::savage_mount::ActiveModel {
            id: NotSet,
            name: Set(savage_mount.clone()),
        }).collect::<Vec<sheef_entities::savage_mount::ActiveModel>>(),
        Err(err) => return Err(anyhow!(err)),
    };

    match sheef_entities::savage_mount::Entity::insert_many(savage_mounts_to_migrate).exec_without_returning(db).await {
        Ok(res) => println!("Migrated {res} savage mounts"),
        Err(err) => return Err(anyhow!(err))
    }

    let migrated_savage_mounts = match sheef_entities::savage_mount::Entity::find().all(db).await {
        Ok(savage_mounts) => savage_mounts,
        Err(err) => return Err(anyhow!(err))
    };

    let mut savage_mounts_to_users = vec![];
    for migrated_savage_mount in migrated_savage_mounts {
        let users_for_savage_mount = match sheef_yaml_database::savage_mount::get_users_for_savage_mount(&migrated_savage_mount.name.clone()).await {
            Ok(users) => users,
            Err(err) => return Err(anyhow!(err)),
        };

        for user in users_for_savage_mount {
            if let Some(u) = users.iter().find(|u| u.username == user) {
                savage_mounts_to_users.push(sheef_entities::savage_mount_to_user::ActiveModel {
                    user_id: Set(u.id),
                    savage_mount_id: Set(migrated_savage_mount.id),
                });
            }
        }
    }

    match sheef_entities::savage_mount_to_user::Entity::insert_many(savage_mounts_to_users).exec_without_returning(db).await {
        Ok(res) => {
            println!("Assigned {res} savage mounts");
            Ok(())
        }
        Err(err) => Err(anyhow!(err))
    }
}

async fn migrate_calendar(db: &DatabaseTransaction, users: Vec<sheef_entities::user::Model>) -> anyhow::Result<()> {
    let calendar = match sheef_yaml_database::event::get_full_calendar().await {
        Ok(calendar) => calendar,
        Err(err) => {
            println!("Failed to load calendar {err}");
            return Err(anyhow!(err));
        }
    };

    let events = calendar.days
        .iter()
        .flat_map(|day| day.events.clone())
        .collect::<Vec<sheef_yaml_entities::Event>>();

    let mut events_to_migrate = vec![];
    for event in events {
        if let Some(user) = users.iter().find(|user| user.username.clone() == event.username.clone()) {
            events_to_migrate.push(sheef_entities::event::ActiveModel {
                id: NotSet,
                user_id: Set(user.id),
                time: Set(event.time),
                date: Set(event.date),
                available: Set(event.available),
            });
        }
    }

    match sheef_entities::event::Entity::insert_many(events_to_migrate).exec_without_returning(db).await {
        Ok(res) => {
            println!("Migrated {res} events");
            Ok(())
        }
        Err(err) => Err(anyhow!(err))
    }
}

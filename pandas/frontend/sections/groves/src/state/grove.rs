use crate::api;
use bamboo_common::core::entities::Grove;
use bounce::{use_atom, Atom, UseAtomHandle};
use yew::hook;
use yew_hooks::use_mount;

#[derive(Atom, PartialEq, Default)]
pub struct GrovesAtom {
    pub groves: Vec<Grove>,
}

#[hook]
pub fn use_groves() -> UseAtomHandle<GrovesAtom> {
    let groves_atom = use_atom::<GrovesAtom>();

    {
        let groves_atom = groves_atom.clone();

        use_mount(move || {
            let groves_atom = groves_atom.clone();

            yew::platform::spawn_local(async move {
                if let Ok(groves) = api::get_groves().await {
                    groves_atom.set(GrovesAtom { groves })
                }
            })
        });
    }

    groves_atom
}

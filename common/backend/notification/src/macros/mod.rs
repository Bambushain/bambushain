#[macro_export]
macro_rules! impl_nats {
    ($type: ty) => {
        use crate::NotificationError;

        impl IntoBytes for $type {
            fn into_bytes(self) -> Result<Bytes, NotificationError> {
                let mut data = Vec::<u8>::new();

                ciborium::into_writer(&self, &mut data)
                    .map_err(|err| NotificationError::new(err.to_string()))?;

                Ok(Bytes::copy_from_slice(data.as_slice()))
            }
        }

        impl FromMessage<$type> for $type {
            fn from_message(message: Message) -> Result<$type, NotificationError> {
                ciborium::from_reader(message.payload.iter().as_slice())
                    .map_err(|err| NotificationError::new(err.to_string()))
            }
        }
    };
}

use serde::{Deserialize, Serialize};
trait Syn {
    type Adapter: Serialize + for<'de> Deserialize<'de>;

    fn to_adapter(&self) -> Self::Adapter;

    // fn from_adapter(adapter: &Self::Adapter) -> Self;
}

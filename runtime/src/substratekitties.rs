use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    //?
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
        MyU32: u32;
        Mybool get(my_bool_getter): bool;
        Value: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn my_function(origin, input_bool: bool)-> Result{
            let sender = ensure_signed(origin)?;
            Ok(())
        }
        fn set_value(origin, value:u64)-> Result{
            let sender = ensure_signed(origin)?;
            //?
            <Value<T>>::put(value);

            Ok(())
        }
    }
}

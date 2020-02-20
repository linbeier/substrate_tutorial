use support::{decl_storage, decl_module, StorageValue, dispatch::Result, StorageMap};
use system::ensure_signed;
//use system::ensure_inherent;

//pub trait Trait: system::Trait {}
pub trait Trait: balances::Trait {}


decl_storage! {
    //?
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
        MyU32: u32;
        Mybool get(my_bool_getter): bool;
        Value: u64;
        SomeValue get(some_value_getter): map T::AccountId => u64;
        MyValue: map T::AccountId => u64;
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

        fn set_value_2(origin, value:u64) -> Result{
            let sender = ensure_signed(origin)?;

            <MyValue<T>>::insert(sender, value);

            Ok(())
        }

        fn get_value(origin) -> Result{
            let sender = ensure_signed(origin)?;
            let ret = <SomeValue<T>>::get(sender);
            //let ret = Self::some_value_getter(sender);
            //return value?
            Ok(())
        }
    }
}

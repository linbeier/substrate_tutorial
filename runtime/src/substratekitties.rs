use support::{decl_storage, decl_module, StorageValue, dispatch::Result, StorageMap};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};

//pub trait Trait: system::Trait {}
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Trace<Hash>
{
    id: Hash,
    from: Hash,
    to: Hash,
    message: u64,
}

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
        Mytrace get(owner): map T::AccountId => Trace<T::Hash>;
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

        fn create_trace(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let new_trace = Trace {
                id: <T as system::Trait>::Hashing::hash_of(&0),
                from: <T as system::Trait>::Hashing::hash_of(&0),
                to: <T as system::Trait>::Hashing::hash_of(&0),
                message: 0,
            };

            <Mytrace<T>>::insert(&sender, new_trace);

            Ok(())

        }
    }
}

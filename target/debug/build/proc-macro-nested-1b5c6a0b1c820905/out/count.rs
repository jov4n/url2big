
        #[doc(hidden)]
        #[macro_export]
        macro_rules! count {
    () => { proc_macro_call_0!() };(!) => { proc_macro_call_1!() };(!!) => { proc_macro_call_2!() };(!!!) => { proc_macro_call_3!() };(!!!!) => { proc_macro_call_4!() };(!!!!!) => { proc_macro_call_5!() };(!!!!!!) => { proc_macro_call_6!() };(!!!!!!!) => { proc_macro_call_7!() };(!!!!!!!!) => { proc_macro_call_8!() };(!!!!!!!!!) => { proc_macro_call_9!() };(!!!!!!!!!!) => { proc_macro_call_10!() };(!!!!!!!!!!!) => { proc_macro_call_11!() };(!!!!!!!!!!!!) => { proc_macro_call_12!() };(!!!!!!!!!!!!!) => { proc_macro_call_13!() };(!!!!!!!!!!!!!!) => { proc_macro_call_14!() };(!!!!!!!!!!!!!!!) => { proc_macro_call_15!() };(!!!!!!!!!!!!!!!!) => { proc_macro_call_16!() };(!!!!!!!!!!!!!!!!!) => { proc_macro_call_17!() };(!!!!!!!!!!!!!!!!!!) => { proc_macro_call_18!() };(!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_19!() };(!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_20!() };(!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_21!() };(!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_22!() };(!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_23!() };(!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_24!() };(!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_25!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_26!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_27!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_28!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_29!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_30!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_31!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_32!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_33!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_34!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_35!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_36!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_37!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_38!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_39!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_40!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_41!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_42!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_43!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_44!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_45!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_46!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_47!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_48!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_49!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_50!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_51!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_52!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_53!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_54!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_55!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_56!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_57!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_58!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_59!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_60!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_61!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_62!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_63!() };(!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!) => { proc_macro_call_64!() };
            ($(!)+) => {
                compile_error!("this macro does not support >64 nested macro invocations")
            };
        }
    
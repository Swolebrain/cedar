#![feature(prelude_import)]
#![forbid(unsafe_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod evaluator {
    //! This module contains the entry point to the wasm isAuthorized functionality.
    use cedar_policy::frontend::{
        is_authorized::json_is_authorized, utils::InterfaceResult,
    };
    use wasm_bindgen::prelude::*;
    #[allow(dead_code)]
    pub fn wasm_is_authorized(input: &str) -> InterfaceResult {
        json_is_authorized(input)
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "isAuthorized"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_isAuthorized(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <InterfaceResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let _ret = wasm_is_authorized(arg0);
                _ret
            };
            <InterfaceResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_isAuthorized() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <InterfaceResult as WasmDescribe>::describe();
            <InterfaceResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 122usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}D\x00\x00\x00\x01\x00\x00\x00\x01\x05input\x00\x0cisAuthorized\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
}
mod formatter {
    use cedar_policy_formatter::{policies_str_to_pretty, Config};
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;
    #[tsify(from_wasm_abi, into_wasm_abi)]
    pub struct FormattingResult {
        success: bool,
        #[serde(rename = "formattedPolicy", skip_serializing_if = "Option::is_none")]
        formatted_policy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    }
    #[automatically_derived]
    const _: () = {
        extern crate serde as _serde;
        use tsify::Tsify;
        use wasm_bindgen::{
            convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
            describe::WasmDescribe, prelude::*,
        };
        #[automatically_derived]
        ///
        #[repr(transparent)]
        pub struct JsType {
            obj: wasm_bindgen::JsValue,
        }
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::convert::TryFromJsValue;
            use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
            use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
            use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
            use wasm_bindgen::describe::WasmDescribe;
            use wasm_bindgen::{JsValue, JsCast, JsObject};
            use wasm_bindgen::__rt::core;
            impl WasmDescribe for JsType {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(NAMED_EXTERNREF);
                    inform(16u32);
                    inform(70u32);
                    inform(111u32);
                    inform(114u32);
                    inform(109u32);
                    inform(97u32);
                    inform(116u32);
                    inform(116u32);
                    inform(105u32);
                    inform(110u32);
                    inform(103u32);
                    inform(82u32);
                    inform(101u32);
                    inform(115u32);
                    inform(117u32);
                    inform(108u32);
                    inform(116u32);
                }
            }
            impl IntoWasmAbi for JsType {
                type Abi = <JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    self.obj.into_abi()
                }
            }
            impl OptionIntoWasmAbi for JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl<'a> OptionIntoWasmAbi for &'a JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl FromWasmAbi for JsType {
                type Abi = <JsValue as FromWasmAbi>::Abi;
                #[inline]
                unsafe fn from_abi(js: Self::Abi) -> Self {
                    JsType {
                        obj: JsValue::from_abi(js).into(),
                    }
                }
            }
            impl OptionFromWasmAbi for JsType {
                #[inline]
                fn is_none(abi: &Self::Abi) -> bool {
                    *abi == 0
                }
            }
            impl<'a> IntoWasmAbi for &'a JsType {
                type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    (&self.obj).into_abi()
                }
            }
            impl RefFromWasmAbi for JsType {
                type Abi = <JsValue as RefFromWasmAbi>::Abi;
                type Anchor = core::mem::ManuallyDrop<JsType>;
                #[inline]
                unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                    core::mem::ManuallyDrop::new(JsType {
                        obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                    })
                }
            }
            impl LongRefFromWasmAbi for JsType {
                type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
                type Anchor = JsType;
                #[inline]
                unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                    JsType { obj: tmp.into() }
                }
            }
            impl From<JsValue> for JsType {
                #[inline]
                fn from(obj: JsValue) -> JsType {
                    JsType { obj: obj.into() }
                }
            }
            impl AsRef<JsValue> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsValue {
                    self.obj.as_ref()
                }
            }
            impl AsRef<JsType> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsType {
                    self
                }
            }
            impl From<JsType> for JsValue {
                #[inline]
                fn from(obj: JsType) -> JsValue {
                    obj.obj.into()
                }
            }
            impl JsCast for JsType {
                fn instanceof(val: &JsValue) -> bool {
                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(
                        all(
                            target_arch = "wasm32",
                            not(any(target_os = "emscripten", target_os = "wasi"))
                        )
                    )]
                    extern "C" {
                        fn __wbg_instanceof_JsType_50645fbbdb6fda8b(val: u32) -> u32;
                    }
                    unsafe {
                        let idx = val.into_abi();
                        __wbg_instanceof_JsType_50645fbbdb6fda8b(idx) != 0
                    }
                }
                #[inline]
                fn unchecked_from_js(val: JsValue) -> Self {
                    JsType { obj: val.into() }
                }
                #[inline]
                fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                    unsafe { &*(val as *const JsValue as *const JsType) }
                }
            }
            impl JsObject for JsType {}
        };
        #[automatically_derived]
        impl core::ops::Deref for JsType {
            type Target = wasm_bindgen::JsValue;
            #[inline]
            fn deref(&self) -> &wasm_bindgen::JsValue {
                &self.obj
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 143usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}Y\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_50645fbbdb6fda8b\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl Tsify for FormattingResult {
            type JsType = JsType;
            const DECL: &'static str = "export interface FormattingResult {\n    success: boolean;\n    formattedPolicy?: string;\n    error?: string;\n}";
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 201usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}\x93\x00\x00\x00\x00\x00\x00\x00\x01mexport interface FormattingResult {\n    success: boolean;\n    formattedPolicy?: string;\n    error?: string;\n}\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl WasmDescribe for FormattingResult {
            #[inline]
            fn describe() {
                <Self as Tsify>::JsType::describe()
            }
        }
        impl IntoWasmAbi for FormattingResult
        where
            Self: _serde::Serialize,
        {
            type Abi = <JsType as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.into_js().unwrap_throw().into_abi()
            }
        }
        impl OptionIntoWasmAbi for FormattingResult
        where
            Self: _serde::Serialize,
        {
            #[inline]
            fn none() -> Self::Abi {
                <JsType as OptionIntoWasmAbi>::none()
            }
        }
        impl FromWasmAbi for FormattingResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            type Abi = <JsType as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                let result = Self::from_js(&JsType::from_abi(js));
                if let Err(err) = result {
                    wasm_bindgen::throw_str(err.to_string().as_ref());
                }
                result.unwrap_throw()
            }
        }
        impl OptionFromWasmAbi for FormattingResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            #[inline]
            fn is_none(js: &Self::Abi) -> bool {
                <JsType as OptionFromWasmAbi>::is_none(js)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for FormattingResult {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "FormattingResult",
                "success",
                &self.success,
                "formatted_policy",
                &self.formatted_policy,
                "error",
                &&self.error,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for FormattingResult {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "FormattingResult",
                    false as usize + 1
                        + if Option::is_none(&self.formatted_policy) { 0 } else { 1 }
                        + if Option::is_none(&self.error) { 0 } else { 1 },
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "success",
                    &self.success,
                )?;
                if !Option::is_none(&self.formatted_policy) {
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "formattedPolicy",
                        &self.formatted_policy,
                    )?;
                } else {
                    _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "formattedPolicy",
                    )?;
                }
                if !Option::is_none(&self.error) {
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "error",
                        &self.error,
                    )?;
                } else {
                    _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "error",
                    )?;
                }
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for FormattingResult {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "success" => _serde::__private::Ok(__Field::__field0),
                            "formattedPolicy" => _serde::__private::Ok(__Field::__field1),
                            "error" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"success" => _serde::__private::Ok(__Field::__field0),
                            b"formattedPolicy" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            b"error" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<FormattingResult>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = FormattingResult;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct FormattingResult",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct FormattingResult with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct FormattingResult with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct FormattingResult with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(FormattingResult {
                            success: __field0,
                            formatted_policy: __field1,
                            error: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "success",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "formattedPolicy",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("success")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("formattedPolicy")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("error")?
                            }
                        };
                        _serde::__private::Ok(FormattingResult {
                            success: __field0,
                            formatted_policy: __field1,
                            error: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "success",
                    "formattedPolicy",
                    "error",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "FormattingResult",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<FormattingResult>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(dead_code)]
    pub fn format_policies(
        policies_str: &str,
        line_width: i32,
        indent_width: i32,
    ) -> FormattingResult {
        let line_width: Result<usize, _> = line_width.try_into();
        let indent_width: Result<isize, _> = indent_width.try_into();
        if line_width.is_err() || indent_width.is_err() {
            return FormattingResult {
                success: false,
                formatted_policy: None,
                error: Some("Input size error (line or indent width)".to_string()),
            };
        }
        let config = Config {
            line_width: line_width.unwrap(),
            indent_width: indent_width.unwrap(),
        };
        match policies_str_to_pretty(policies_str, &config) {
            Ok(prettified_policy) => {
                FormattingResult {
                    success: true,
                    formatted_policy: Some(prettified_policy),
                    error: None,
                }
            }
            Err(err) => {
                FormattingResult {
                    success: false,
                    formatted_policy: None,
                    error: Some(err.to_string()),
                }
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "formatPolicies"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_formatPolicies(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg2_1: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg2_2: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg2_3: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg2_4: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <FormattingResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let arg1 = unsafe {
                    <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg1_1,
                            arg1_2,
                            arg1_3,
                            arg1_4,
                        ),
                    )
                };
                let arg2 = unsafe {
                    <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg2_1,
                            arg2_2,
                            arg2_3,
                            arg2_4,
                        ),
                    )
                };
                let _ret = format_policies(arg0, arg1, arg2);
                _ret
            };
            <FormattingResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_formatPolicies() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(3u32);
            <&str as WasmDescribe>::describe();
            <i32 as WasmDescribe>::describe();
            <i32 as WasmDescribe>::describe();
            <FormattingResult as WasmDescribe>::describe();
            <FormattingResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 155usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}e\x00\x00\x00\x01\x00\x00\x00\x03\x0cpolicies_str\nline_width\x0cindent_width\x00\x0eformatPolicies\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
}
mod policies_and_templates {
    use std::str::FromStr;
    use cedar_policy::{Policy, PolicySet};
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;
    #[serde(rename_all = "camelCase")]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub enum JsonToPolicyResult {
        Success { policy_text: String },
        Error { errors: Vec<String> },
    }
    #[automatically_derived]
    const _: () = {
        extern crate serde as _serde;
        use tsify::Tsify;
        use wasm_bindgen::{
            convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
            describe::WasmDescribe, prelude::*,
        };
        #[automatically_derived]
        ///
        #[repr(transparent)]
        pub struct JsType {
            obj: wasm_bindgen::JsValue,
        }
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::convert::TryFromJsValue;
            use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
            use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
            use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
            use wasm_bindgen::describe::WasmDescribe;
            use wasm_bindgen::{JsValue, JsCast, JsObject};
            use wasm_bindgen::__rt::core;
            impl WasmDescribe for JsType {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(NAMED_EXTERNREF);
                    inform(18u32);
                    inform(74u32);
                    inform(115u32);
                    inform(111u32);
                    inform(110u32);
                    inform(84u32);
                    inform(111u32);
                    inform(80u32);
                    inform(111u32);
                    inform(108u32);
                    inform(105u32);
                    inform(99u32);
                    inform(121u32);
                    inform(82u32);
                    inform(101u32);
                    inform(115u32);
                    inform(117u32);
                    inform(108u32);
                    inform(116u32);
                }
            }
            impl IntoWasmAbi for JsType {
                type Abi = <JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    self.obj.into_abi()
                }
            }
            impl OptionIntoWasmAbi for JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl<'a> OptionIntoWasmAbi for &'a JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl FromWasmAbi for JsType {
                type Abi = <JsValue as FromWasmAbi>::Abi;
                #[inline]
                unsafe fn from_abi(js: Self::Abi) -> Self {
                    JsType {
                        obj: JsValue::from_abi(js).into(),
                    }
                }
            }
            impl OptionFromWasmAbi for JsType {
                #[inline]
                fn is_none(abi: &Self::Abi) -> bool {
                    *abi == 0
                }
            }
            impl<'a> IntoWasmAbi for &'a JsType {
                type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    (&self.obj).into_abi()
                }
            }
            impl RefFromWasmAbi for JsType {
                type Abi = <JsValue as RefFromWasmAbi>::Abi;
                type Anchor = core::mem::ManuallyDrop<JsType>;
                #[inline]
                unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                    core::mem::ManuallyDrop::new(JsType {
                        obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                    })
                }
            }
            impl LongRefFromWasmAbi for JsType {
                type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
                type Anchor = JsType;
                #[inline]
                unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                    JsType { obj: tmp.into() }
                }
            }
            impl From<JsValue> for JsType {
                #[inline]
                fn from(obj: JsValue) -> JsType {
                    JsType { obj: obj.into() }
                }
            }
            impl AsRef<JsValue> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsValue {
                    self.obj.as_ref()
                }
            }
            impl AsRef<JsType> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsType {
                    self
                }
            }
            impl From<JsType> for JsValue {
                #[inline]
                fn from(obj: JsType) -> JsValue {
                    obj.obj.into()
                }
            }
            impl JsCast for JsType {
                fn instanceof(val: &JsValue) -> bool {
                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(
                        all(
                            target_arch = "wasm32",
                            not(any(target_os = "emscripten", target_os = "wasi"))
                        )
                    )]
                    extern "C" {
                        fn __wbg_instanceof_JsType_50645fbbdb6fda8b(val: u32) -> u32;
                    }
                    unsafe {
                        let idx = val.into_abi();
                        __wbg_instanceof_JsType_50645fbbdb6fda8b(idx) != 0
                    }
                }
                #[inline]
                fn unchecked_from_js(val: JsValue) -> Self {
                    JsType { obj: val.into() }
                }
                #[inline]
                fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                    unsafe { &*(val as *const JsValue as *const JsType) }
                }
            }
            impl JsObject for JsType {}
        };
        #[automatically_derived]
        impl core::ops::Deref for JsType {
            type Target = wasm_bindgen::JsValue;
            #[inline]
            fn deref(&self) -> &wasm_bindgen::JsValue {
                &self.obj
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 143usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}Y\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_50645fbbdb6fda8b\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl Tsify for JsonToPolicyResult {
            type JsType = JsType;
            const DECL: &'static str = "export type JsonToPolicyResult = { success: { policy_text: string } } | { error: { errors: string[] } };";
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 196usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}\x8e\x00\x00\x00\x00\x00\x00\x00\x01hexport type JsonToPolicyResult = { success: { policy_text: string } } | { error: { errors: string[] } };\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl WasmDescribe for JsonToPolicyResult {
            #[inline]
            fn describe() {
                <Self as Tsify>::JsType::describe()
            }
        }
        impl IntoWasmAbi for JsonToPolicyResult
        where
            Self: _serde::Serialize,
        {
            type Abi = <JsType as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.into_js().unwrap_throw().into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsonToPolicyResult
        where
            Self: _serde::Serialize,
        {
            #[inline]
            fn none() -> Self::Abi {
                <JsType as OptionIntoWasmAbi>::none()
            }
        }
        impl FromWasmAbi for JsonToPolicyResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            type Abi = <JsType as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                let result = Self::from_js(&JsType::from_abi(js));
                if let Err(err) = result {
                    wasm_bindgen::throw_str(err.to_string().as_ref());
                }
                result.unwrap_throw()
            }
        }
        impl OptionFromWasmAbi for JsonToPolicyResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            #[inline]
            fn is_none(js: &Self::Abi) -> bool {
                <JsType as OptionFromWasmAbi>::is_none(js)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for JsonToPolicyResult {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                JsonToPolicyResult::Success { policy_text: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Success",
                        "policy_text",
                        &__self_0,
                    )
                }
                JsonToPolicyResult::Error { errors: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Error",
                        "errors",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for JsonToPolicyResult {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    JsonToPolicyResult::Success { ref policy_text } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "JsonToPolicyResult",
                            0u32,
                            "success",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "policy_text",
                            policy_text,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    JsonToPolicyResult::Error { ref errors } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "JsonToPolicyResult",
                            1u32,
                            "error",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "errors",
                            errors,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for JsonToPolicyResult {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "success" => _serde::__private::Ok(__Field::__field0),
                            "error" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"success" => _serde::__private::Ok(__Field::__field0),
                            b"error" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<JsonToPolicyResult>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = JsonToPolicyResult;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum JsonToPolicyResult",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "policy_text" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"policy_text" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<JsonToPolicyResult>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = JsonToPolicyResult;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant JsonToPolicyResult::Success",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            String,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant JsonToPolicyResult::Success with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(JsonToPolicyResult::Success {
                                            policy_text: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "policy_text",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("policy_text")?
                                            }
                                        };
                                        _serde::__private::Ok(JsonToPolicyResult::Success {
                                            policy_text: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["policy_text"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            JsonToPolicyResult,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "errors" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"errors" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<JsonToPolicyResult>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = JsonToPolicyResult;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant JsonToPolicyResult::Error",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            Vec<String>,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant JsonToPolicyResult::Error with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(JsonToPolicyResult::Error {
                                            errors: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("errors"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            Vec<String>,
                                                        >(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("errors")?
                                            }
                                        };
                                        _serde::__private::Ok(JsonToPolicyResult::Error {
                                            errors: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["errors"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            JsonToPolicyResult,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["success", "error"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "JsonToPolicyResult",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<JsonToPolicyResult>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(dead_code)]
    pub fn policy_text_from_json(json_str: &str) -> JsonToPolicyResult {
        let parsed_json = serde_json::from_str(json_str)
            .expect("Deserialization failed");
        let policy = Policy::from_json(None, parsed_json);
        match policy {
            Ok(p) => {
                JsonToPolicyResult::Success {
                    policy_text: p.to_string(),
                }
            }
            Err(e) => {
                JsonToPolicyResult::Error {
                    errors: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([e.to_string()]),
                    ),
                }
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "policyTextFromJson"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_policyTextFromJson(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <JsonToPolicyResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let _ret = policy_text_from_json(arg0);
                _ret
            };
            <JsonToPolicyResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_policyTextFromJson() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <JsonToPolicyResult as WasmDescribe>::describe();
            <JsonToPolicyResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 131usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}M\x00\x00\x00\x01\x00\x00\x00\x01\x08json_str\x00\x12policyTextFromJson\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
    #[serde(untagged)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub enum PolicyToJsonResult {
        Success { policy: cedar_policy_core::est::Policy },
        Error { errors: Vec<String> },
    }
    #[automatically_derived]
    const _: () = {
        extern crate serde as _serde;
        use tsify::Tsify;
        use wasm_bindgen::{
            convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
            describe::WasmDescribe, prelude::*,
        };
        #[automatically_derived]
        ///
        #[repr(transparent)]
        pub struct JsType {
            obj: wasm_bindgen::JsValue,
        }
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::convert::TryFromJsValue;
            use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
            use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
            use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
            use wasm_bindgen::describe::WasmDescribe;
            use wasm_bindgen::{JsValue, JsCast, JsObject};
            use wasm_bindgen::__rt::core;
            impl WasmDescribe for JsType {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(NAMED_EXTERNREF);
                    inform(18u32);
                    inform(80u32);
                    inform(111u32);
                    inform(108u32);
                    inform(105u32);
                    inform(99u32);
                    inform(121u32);
                    inform(84u32);
                    inform(111u32);
                    inform(74u32);
                    inform(115u32);
                    inform(111u32);
                    inform(110u32);
                    inform(82u32);
                    inform(101u32);
                    inform(115u32);
                    inform(117u32);
                    inform(108u32);
                    inform(116u32);
                }
            }
            impl IntoWasmAbi for JsType {
                type Abi = <JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    self.obj.into_abi()
                }
            }
            impl OptionIntoWasmAbi for JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl<'a> OptionIntoWasmAbi for &'a JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl FromWasmAbi for JsType {
                type Abi = <JsValue as FromWasmAbi>::Abi;
                #[inline]
                unsafe fn from_abi(js: Self::Abi) -> Self {
                    JsType {
                        obj: JsValue::from_abi(js).into(),
                    }
                }
            }
            impl OptionFromWasmAbi for JsType {
                #[inline]
                fn is_none(abi: &Self::Abi) -> bool {
                    *abi == 0
                }
            }
            impl<'a> IntoWasmAbi for &'a JsType {
                type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    (&self.obj).into_abi()
                }
            }
            impl RefFromWasmAbi for JsType {
                type Abi = <JsValue as RefFromWasmAbi>::Abi;
                type Anchor = core::mem::ManuallyDrop<JsType>;
                #[inline]
                unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                    core::mem::ManuallyDrop::new(JsType {
                        obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                    })
                }
            }
            impl LongRefFromWasmAbi for JsType {
                type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
                type Anchor = JsType;
                #[inline]
                unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                    JsType { obj: tmp.into() }
                }
            }
            impl From<JsValue> for JsType {
                #[inline]
                fn from(obj: JsValue) -> JsType {
                    JsType { obj: obj.into() }
                }
            }
            impl AsRef<JsValue> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsValue {
                    self.obj.as_ref()
                }
            }
            impl AsRef<JsType> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsType {
                    self
                }
            }
            impl From<JsType> for JsValue {
                #[inline]
                fn from(obj: JsType) -> JsValue {
                    obj.obj.into()
                }
            }
            impl JsCast for JsType {
                fn instanceof(val: &JsValue) -> bool {
                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(
                        all(
                            target_arch = "wasm32",
                            not(any(target_os = "emscripten", target_os = "wasi"))
                        )
                    )]
                    extern "C" {
                        fn __wbg_instanceof_JsType_50645fbbdb6fda8b(val: u32) -> u32;
                    }
                    unsafe {
                        let idx = val.into_abi();
                        __wbg_instanceof_JsType_50645fbbdb6fda8b(idx) != 0
                    }
                }
                #[inline]
                fn unchecked_from_js(val: JsValue) -> Self {
                    JsType { obj: val.into() }
                }
                #[inline]
                fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                    unsafe { &*(val as *const JsValue as *const JsType) }
                }
            }
            impl JsObject for JsType {}
        };
        #[automatically_derived]
        impl core::ops::Deref for JsType {
            type Target = wasm_bindgen::JsValue;
            #[inline]
            fn deref(&self) -> &wasm_bindgen::JsValue {
                &self.obj
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 143usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}Y\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_50645fbbdb6fda8b\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl Tsify for PolicyToJsonResult {
            type JsType = JsType;
            const DECL: &'static str = "export type PolicyToJsonResult = { policy: Policy } | { errors: string[] };";
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 167usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}q\x00\x00\x00\x00\x00\x00\x00\x01Kexport type PolicyToJsonResult = { policy: Policy } | { errors: string[] };\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl WasmDescribe for PolicyToJsonResult {
            #[inline]
            fn describe() {
                <Self as Tsify>::JsType::describe()
            }
        }
        impl IntoWasmAbi for PolicyToJsonResult
        where
            Self: _serde::Serialize,
        {
            type Abi = <JsType as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.into_js().unwrap_throw().into_abi()
            }
        }
        impl OptionIntoWasmAbi for PolicyToJsonResult
        where
            Self: _serde::Serialize,
        {
            #[inline]
            fn none() -> Self::Abi {
                <JsType as OptionIntoWasmAbi>::none()
            }
        }
        impl FromWasmAbi for PolicyToJsonResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            type Abi = <JsType as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                let result = Self::from_js(&JsType::from_abi(js));
                if let Err(err) = result {
                    wasm_bindgen::throw_str(err.to_string().as_ref());
                }
                result.unwrap_throw()
            }
        }
        impl OptionFromWasmAbi for PolicyToJsonResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            #[inline]
            fn is_none(js: &Self::Abi) -> bool {
                <JsType as OptionFromWasmAbi>::is_none(js)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for PolicyToJsonResult {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PolicyToJsonResult::Success { policy: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Success",
                        "policy",
                        &__self_0,
                    )
                }
                PolicyToJsonResult::Error { errors: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Error",
                        "errors",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PolicyToJsonResult {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    PolicyToJsonResult::Success { ref policy } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "PolicyToJsonResult",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "policy",
                            policy,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                    PolicyToJsonResult::Error { ref errors } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "PolicyToJsonResult",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "errors",
                            errors,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PolicyToJsonResult {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content = <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                    __deserializer,
                )?;
                let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                    __D::Error,
                >::new(&__content);
                if let _serde::__private::Ok(__ok) = {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "policy" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"policy" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<PolicyToJsonResult>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PolicyToJsonResult;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct variant PolicyToJsonResult::Success",
                            )
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                cedar_policy_core::est::Policy,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("policy"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                cedar_policy_core::est::Policy,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("policy")?
                                }
                            };
                            _serde::__private::Ok(PolicyToJsonResult::Success {
                                policy: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["policy"];
                    _serde::Deserializer::deserialize_any(
                        __deserializer,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PolicyToJsonResult>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                } {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "errors" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"errors" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<PolicyToJsonResult>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PolicyToJsonResult;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct variant PolicyToJsonResult::Error",
                            )
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("errors"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("errors")?
                                }
                            };
                            _serde::__private::Ok(PolicyToJsonResult::Error {
                                errors: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["errors"];
                    _serde::Deserializer::deserialize_any(
                        __deserializer,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PolicyToJsonResult>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                } {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(
                    _serde::de::Error::custom(
                        "data did not match any variant of untagged enum PolicyToJsonResult",
                    ),
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for PolicyToJsonResult {
        #[inline]
        fn clone(&self) -> PolicyToJsonResult {
            match self {
                PolicyToJsonResult::Success { policy: __self_0 } => {
                    PolicyToJsonResult::Success {
                        policy: ::core::clone::Clone::clone(__self_0),
                    }
                }
                PolicyToJsonResult::Error { errors: __self_0 } => {
                    PolicyToJsonResult::Error {
                        errors: ::core::clone::Clone::clone(__self_0),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn policy_text_to_json(cedar_str: &str) -> PolicyToJsonResult {
        let parsed_policy = match Policy::from_str(cedar_str) {
            Ok(policy) => policy,
            Err(errs) => {
                return PolicyToJsonResult::Error {
                    errors: errs.errors_as_strings(),
                };
            }
        };
        match parsed_policy.to_json_policy() {
            Ok(json_policy) => {
                PolicyToJsonResult::Success {
                    policy: json_policy,
                }
            }
            Err(err) => {
                PolicyToJsonResult::Error {
                    errors: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([err.to_string()]),
                    ),
                }
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "policyTextToJson"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_policyTextToJson(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <PolicyToJsonResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let _ret = policy_text_to_json(arg0);
                _ret
            };
            <PolicyToJsonResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_policyTextToJson() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <PolicyToJsonResult as WasmDescribe>::describe();
            <PolicyToJsonResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 130usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}L\x00\x00\x00\x01\x00\x00\x00\x01\tcedar_str\x00\x10policyTextToJson\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
    #[tsify(into_wasm_abi, from_wasm_abi)]
    /// struct that defines the result for the syntax validation function
    pub enum PolicySetParseResult {
        /// represents successful syntax validation
        Success { policies: i32, templates: i32 },
        /// represents a syntax error and encloses a vector of the errors
        SyntaxError { errors: Vec<String> },
    }
    #[automatically_derived]
    const _: () = {
        extern crate serde as _serde;
        use tsify::Tsify;
        use wasm_bindgen::{
            convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
            describe::WasmDescribe, prelude::*,
        };
        #[automatically_derived]
        ///
        #[repr(transparent)]
        pub struct JsType {
            obj: wasm_bindgen::JsValue,
        }
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::convert::TryFromJsValue;
            use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
            use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
            use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
            use wasm_bindgen::describe::WasmDescribe;
            use wasm_bindgen::{JsValue, JsCast, JsObject};
            use wasm_bindgen::__rt::core;
            impl WasmDescribe for JsType {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(NAMED_EXTERNREF);
                    inform(20u32);
                    inform(80u32);
                    inform(111u32);
                    inform(108u32);
                    inform(105u32);
                    inform(99u32);
                    inform(121u32);
                    inform(83u32);
                    inform(101u32);
                    inform(116u32);
                    inform(80u32);
                    inform(97u32);
                    inform(114u32);
                    inform(115u32);
                    inform(101u32);
                    inform(82u32);
                    inform(101u32);
                    inform(115u32);
                    inform(117u32);
                    inform(108u32);
                    inform(116u32);
                }
            }
            impl IntoWasmAbi for JsType {
                type Abi = <JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    self.obj.into_abi()
                }
            }
            impl OptionIntoWasmAbi for JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl<'a> OptionIntoWasmAbi for &'a JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl FromWasmAbi for JsType {
                type Abi = <JsValue as FromWasmAbi>::Abi;
                #[inline]
                unsafe fn from_abi(js: Self::Abi) -> Self {
                    JsType {
                        obj: JsValue::from_abi(js).into(),
                    }
                }
            }
            impl OptionFromWasmAbi for JsType {
                #[inline]
                fn is_none(abi: &Self::Abi) -> bool {
                    *abi == 0
                }
            }
            impl<'a> IntoWasmAbi for &'a JsType {
                type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    (&self.obj).into_abi()
                }
            }
            impl RefFromWasmAbi for JsType {
                type Abi = <JsValue as RefFromWasmAbi>::Abi;
                type Anchor = core::mem::ManuallyDrop<JsType>;
                #[inline]
                unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                    core::mem::ManuallyDrop::new(JsType {
                        obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                    })
                }
            }
            impl LongRefFromWasmAbi for JsType {
                type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
                type Anchor = JsType;
                #[inline]
                unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                    JsType { obj: tmp.into() }
                }
            }
            impl From<JsValue> for JsType {
                #[inline]
                fn from(obj: JsValue) -> JsType {
                    JsType { obj: obj.into() }
                }
            }
            impl AsRef<JsValue> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsValue {
                    self.obj.as_ref()
                }
            }
            impl AsRef<JsType> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsType {
                    self
                }
            }
            impl From<JsType> for JsValue {
                #[inline]
                fn from(obj: JsType) -> JsValue {
                    obj.obj.into()
                }
            }
            impl JsCast for JsType {
                fn instanceof(val: &JsValue) -> bool {
                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(
                        all(
                            target_arch = "wasm32",
                            not(any(target_os = "emscripten", target_os = "wasi"))
                        )
                    )]
                    extern "C" {
                        fn __wbg_instanceof_JsType_50645fbbdb6fda8b(val: u32) -> u32;
                    }
                    unsafe {
                        let idx = val.into_abi();
                        __wbg_instanceof_JsType_50645fbbdb6fda8b(idx) != 0
                    }
                }
                #[inline]
                fn unchecked_from_js(val: JsValue) -> Self {
                    JsType { obj: val.into() }
                }
                #[inline]
                fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                    unsafe { &*(val as *const JsValue as *const JsType) }
                }
            }
            impl JsObject for JsType {}
        };
        #[automatically_derived]
        impl core::ops::Deref for JsType {
            type Target = wasm_bindgen::JsValue;
            #[inline]
            fn deref(&self) -> &wasm_bindgen::JsValue {
                &self.obj
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 143usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}Y\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_50645fbbdb6fda8b\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl Tsify for PolicySetParseResult {
            type JsType = JsType;
            const DECL: &'static str = "export type PolicySetParseResult = { Success: { policies: number; templates: number } } | { SyntaxError: { errors: string[] } };";
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 221usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}\xa7\x00\x00\x00\x00\x00\x00\x00\x01\x80\x01export type PolicySetParseResult = { Success: { policies: number; templates: number } } | { SyntaxError: { errors: string[] } };\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl WasmDescribe for PolicySetParseResult {
            #[inline]
            fn describe() {
                <Self as Tsify>::JsType::describe()
            }
        }
        impl IntoWasmAbi for PolicySetParseResult
        where
            Self: _serde::Serialize,
        {
            type Abi = <JsType as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.into_js().unwrap_throw().into_abi()
            }
        }
        impl OptionIntoWasmAbi for PolicySetParseResult
        where
            Self: _serde::Serialize,
        {
            #[inline]
            fn none() -> Self::Abi {
                <JsType as OptionIntoWasmAbi>::none()
            }
        }
        impl FromWasmAbi for PolicySetParseResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            type Abi = <JsType as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                let result = Self::from_js(&JsType::from_abi(js));
                if let Err(err) = result {
                    wasm_bindgen::throw_str(err.to_string().as_ref());
                }
                result.unwrap_throw()
            }
        }
        impl OptionFromWasmAbi for PolicySetParseResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            #[inline]
            fn is_none(js: &Self::Abi) -> bool {
                <JsType as OptionFromWasmAbi>::is_none(js)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for PolicySetParseResult {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PolicySetParseResult::Success {
                    policies: __self_0,
                    templates: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Success",
                        "policies",
                        __self_0,
                        "templates",
                        &__self_1,
                    )
                }
                PolicySetParseResult::SyntaxError { errors: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SyntaxError",
                        "errors",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PolicySetParseResult {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    PolicySetParseResult::Success { ref policies, ref templates } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "PolicySetParseResult",
                            0u32,
                            "Success",
                            0 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "policies",
                            policies,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "templates",
                            templates,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    PolicySetParseResult::SyntaxError { ref errors } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "PolicySetParseResult",
                            1u32,
                            "SyntaxError",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "errors",
                            errors,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PolicySetParseResult {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Success" => _serde::__private::Ok(__Field::__field0),
                            "SyntaxError" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Success" => _serde::__private::Ok(__Field::__field0),
                            b"SyntaxError" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PolicySetParseResult>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PolicySetParseResult;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum PolicySetParseResult",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "policies" => _serde::__private::Ok(__Field::__field0),
                                            "templates" => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"policies" => _serde::__private::Ok(__Field::__field0),
                                            b"templates" => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<
                                        PolicySetParseResult,
                                    >,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = PolicySetParseResult;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant PolicySetParseResult::Success",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            i32,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant PolicySetParseResult::Success with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            i32,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant PolicySetParseResult::Success with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(PolicySetParseResult::Success {
                                            policies: __field0,
                                            templates: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "policies",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "templates",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("policies")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("templates")?
                                            }
                                        };
                                        _serde::__private::Ok(PolicySetParseResult::Success {
                                            policies: __field0,
                                            templates: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "policies",
                                    "templates",
                                ];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            PolicySetParseResult,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "errors" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"errors" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<
                                        PolicySetParseResult,
                                    >,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = PolicySetParseResult;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant PolicySetParseResult::SyntaxError",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            Vec<String>,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant PolicySetParseResult::SyntaxError with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(PolicySetParseResult::SyntaxError {
                                            errors: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("errors"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<
                                                            Vec<String>,
                                                        >(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("errors")?
                                            }
                                        };
                                        _serde::__private::Ok(PolicySetParseResult::SyntaxError {
                                            errors: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["errors"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            PolicySetParseResult,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Success", "SyntaxError"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "PolicySetParseResult",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PolicySetParseResult>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(dead_code)]
    pub fn parse_policy_set(input_policies_str: &str) -> PolicySetParseResult {
        match PolicySet::from_str(input_policies_str) {
            Err(validation_errors) => {
                PolicySetParseResult::SyntaxError {
                    errors: validation_errors.errors_as_strings(),
                }
            }
            Ok(policy_set) => {
                let policies_count: Result<i32, <i32 as TryFrom<usize>>::Error> = policy_set
                    .policies()
                    .count()
                    .try_into();
                let templates_count: Result<i32, <i32 as TryFrom<usize>>::Error> = policy_set
                    .templates()
                    .count()
                    .try_into();
                match (policies_count, templates_count) {
                    (Ok(p), Ok(t)) => {
                        PolicySetParseResult::Success {
                            policies: p,
                            templates: t,
                        }
                    }
                    _ => {
                        PolicySetParseResult::SyntaxError {
                            errors: <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    "Error counting policies or templates".to_string(),
                                ]),
                            ),
                        }
                    }
                }
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "parsePolicySet"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_parsePolicySet(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <PolicySetParseResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let _ret = parse_policy_set(arg0);
                _ret
            };
            <PolicySetParseResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_parsePolicySet() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <PolicySetParseResult as WasmDescribe>::describe();
            <PolicySetParseResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 137usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}S\x00\x00\x00\x01\x00\x00\x00\x01\x12input_policies_str\x00\x0eparsePolicySet\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
    pub struct Template {
        text: String,
        slots: Vec<String>,
        parse_errors: Option<Vec<String>>,
    }
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribe for Template {
        fn describe() {
            use wasm_bindgen::__wbindgen_if_not_std;
            use wasm_bindgen::describe::*;
            inform(RUST_STRUCT);
            inform(8u32);
            inform(84u32);
            inform(101u32);
            inform(109u32);
            inform(112u32);
            inform(108u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::IntoWasmAbi for Template {
        type Abi = u32;
        fn into_abi(self) -> u32 {
            use wasm_bindgen::__rt::std::boxed::Box;
            use wasm_bindgen::__rt::WasmRefCell;
            Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::FromWasmAbi for Template {
        type Abi = u32;
        unsafe fn from_abi(js: u32) -> Self {
            use wasm_bindgen::__rt::std::boxed::Box;
            use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
            let ptr = js as *mut WasmRefCell<Template>;
            assert_not_null(ptr);
            let js = Box::from_raw(ptr);
            (*js).borrow_mut();
            js.into_inner()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::core::convert::From<Template> for wasm_bindgen::JsValue {
        fn from(value: Template) -> Self {
            let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            extern "C" {
                fn __wbg_template_new(ptr: u32) -> u32;
            }
            unsafe {
                <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    __wbg_template_new(ptr),
                )
            }
        }
    }
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub unsafe extern "C" fn __wbg_template_free(ptr: u32) {
            let _ = <Template as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr);
        }
    };
    #[automatically_derived]
    impl wasm_bindgen::convert::RefFromWasmAbi for Template {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::Ref<'static, Template>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Template>;
            wasm_bindgen::__rt::assert_not_null(js);
            (*js).borrow()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefMutFromWasmAbi for Template {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::RefMut<'static, Template>;
        unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Template>;
            wasm_bindgen::__rt::assert_not_null(js);
            (*js).borrow_mut()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::LongRefFromWasmAbi for Template {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::Ref<'static, Template>;
        unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
            <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionIntoWasmAbi for Template {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionFromWasmAbi for Template {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    #[allow(clippy::all)]
    impl wasm_bindgen::convert::TryFromJsValue for Template {
        type Error = wasm_bindgen::JsValue;
        fn try_from_js_value(
            value: wasm_bindgen::JsValue,
        ) -> wasm_bindgen::__rt::std::result::Result<Self, Self::Error> {
            let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            extern "C" {
                fn __wbg_template_unwrap(ptr: u32) -> u32;
            }
            let ptr = unsafe { __wbg_template_unwrap(idx) };
            if ptr == 0 {
                wasm_bindgen::__rt::std::result::Result::Err(value)
            } else {
                wasm_bindgen::__rt::std::mem::forget(value);
                unsafe {
                    wasm_bindgen::__rt::std::result::Result::Ok(
                        <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                    )
                }
            }
        }
    }
    impl wasm_bindgen::describe::WasmDescribeVector for Template {
        fn describe_vector() {
            use wasm_bindgen::describe::*;
            inform(VECTOR);
            inform(NAMED_EXTERNREF);
            inform(8u32);
            inform(84u32);
            inform(101u32);
            inform(109u32);
            inform(112u32);
            inform(108u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
        }
    }
    impl wasm_bindgen::convert::VectorIntoWasmAbi for Template {
        type Abi = <wasm_bindgen::__rt::std::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
        fn vector_into_abi(
            vector: wasm_bindgen::__rt::std::boxed::Box<[Template]>,
        ) -> Self::Abi {
            wasm_bindgen::convert::js_value_vector_into_abi(vector)
        }
    }
    impl wasm_bindgen::convert::VectorFromWasmAbi for Template {
        type Abi = <wasm_bindgen::__rt::std::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::FromWasmAbi>::Abi;
        unsafe fn vector_from_abi(
            js: Self::Abi,
        ) -> wasm_bindgen::__rt::std::boxed::Box<[Template]> {
            wasm_bindgen::convert::js_value_vector_from_abi(js)
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 104usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}2\x00\x00\x00\x00\x00\x00\x01\x08Template\x00\x00\x00\x01\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Template {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Template",
                "text",
                &self.text,
                "slots",
                &self.slots,
                "parse_errors",
                &&self.parse_errors,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Template {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Template",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "text",
                    &self.text,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "slots",
                    &self.slots,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "parse_errors",
                    &self.parse_errors,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Template {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "text" => _serde::__private::Ok(__Field::__field0),
                            "slots" => _serde::__private::Ok(__Field::__field1),
                            "parse_errors" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"text" => _serde::__private::Ok(__Field::__field0),
                            b"slots" => _serde::__private::Ok(__Field::__field1),
                            b"parse_errors" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Template>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Template;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Template",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Template with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Template with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<Vec<String>>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Template with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Template {
                            text: __field0,
                            slots: __field1,
                            parse_errors: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<Vec<String>>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("text"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("slots"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "parse_errors",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Vec<String>>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("text")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("slots")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("parse_errors")?
                            }
                        };
                        _serde::__private::Ok(Template {
                            text: __field0,
                            slots: __field1,
                            parse_errors: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "text",
                    "slots",
                    "parse_errors",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Template",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Template>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Template {
        pub fn new(template_str: &str) -> Template {
            #[automatically_derived]
            const _: () = {
                #[export_name = "template_new"]
                pub unsafe extern "C" fn __wasm_bindgen_generated_Template_new(
                    arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                    arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                    arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                    arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
                ) -> wasm_bindgen::convert::WasmRet<
                    <Template as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
                > {
                    let _ret = {
                        let arg0 = unsafe {
                            <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let arg0 = &*arg0;
                        let _ret = Template::new(arg0);
                        _ret
                    };
                    <Template as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                        .into()
                }
            };
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            #[automatically_derived]
            const _: () = {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C" fn __wbindgen_describe_template_new() {
                    use wasm_bindgen::describe::*;
                    wasm_bindgen::__rt::link_mem_intrinsics();
                    inform(FUNCTION);
                    inform(0);
                    inform(1u32);
                    <&str as WasmDescribe>::describe();
                    <Template as WasmDescribe>::describe();
                    <Template as WasmDescribe>::describe();
                }
            };
            #[cfg(target_arch = "wasm32")]
            #[automatically_derived]
            const _: () = {
                static _INCLUDED_FILES: &[&str] = &[];
                #[link_section = "__wasm_bindgen_unstable"]
                pub static _GENERATED: [u8; 127usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}I\x00\x00\x00\x01\x01\x08Template\x00\x00\x01\x0ctemplate_str\x00\x03new\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
            };
            match cedar_policy::Template::from_str(template_str) {
                Err(parse_errs) => {
                    Self {
                        text: String::from(""),
                        slots: ::alloc::vec::Vec::new(),
                        parse_errors: Some(parse_errs.errors_as_strings()),
                    }
                }
                Ok(template) => {
                    match template.slots().count() {
                        1 | 2 => {
                            Self {
                                text: template_str.to_string(),
                                slots: template
                                    .slots()
                                    .map(|slot| slot.to_string())
                                    .collect(),
                                parse_errors: None,
                            }
                        }
                        _ => {
                            Self {
                                text: String::from(""),
                                slots: ::alloc::vec::Vec::new(),
                                parse_errors: Some(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            "Expected template to have 1 or 2 slots".to_string(),
                                        ]),
                                    ),
                                ),
                            }
                        }
                    }
                }
            }
        }
        pub fn is_valid(&self) -> bool {
            #[automatically_derived]
            const _: () = {
                #[export_name = "template_isValid"]
                pub unsafe extern "C" fn __wasm_bindgen_generated_Template_isValid(
                    me: u32,
                ) -> wasm_bindgen::convert::WasmRet<
                    <bool as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
                > {
                    let _ret = {
                        let me = unsafe {
                            <Template as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                me,
                            )
                        };
                        let me = &*me;
                        let _ret = me.is_valid();
                        _ret
                    };
                    <bool as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                        .into()
                }
            };
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            #[automatically_derived]
            const _: () = {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C" fn __wbindgen_describe_template_isValid() {
                    use wasm_bindgen::describe::*;
                    wasm_bindgen::__rt::link_mem_intrinsics();
                    inform(FUNCTION);
                    inform(0);
                    inform(0u32);
                    <bool as WasmDescribe>::describe();
                    <bool as WasmDescribe>::describe();
                }
            };
            #[cfg(target_arch = "wasm32")]
            #[automatically_derived]
            const _: () = {
                static _INCLUDED_FILES: &[&str] = &[];
                #[link_section = "__wasm_bindgen_unstable"]
                pub static _GENERATED: [u8; 120usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}B\x00\x00\x00\x01\x01\x08Template\x00\x00\x00\x00\x07isValid\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
            };
            self.parse_errors.is_none()
        }
        pub fn parse_errors(&self) -> Option<Vec<String>> {
            #[automatically_derived]
            const _: () = {
                #[export_name = "template_parseErrors"]
                pub unsafe extern "C" fn __wasm_bindgen_generated_Template_parseErrors(
                    me: u32,
                ) -> wasm_bindgen::convert::WasmRet<
                    <Option<Vec<String>> as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
                > {
                    let _ret = {
                        let me = unsafe {
                            <Template as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                me,
                            )
                        };
                        let me = &*me;
                        let _ret = me.parse_errors();
                        _ret
                    };
                    <Option<
                        Vec<String>,
                    > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                        .into()
                }
            };
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            #[automatically_derived]
            const _: () = {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C" fn __wbindgen_describe_template_parseErrors() {
                    use wasm_bindgen::describe::*;
                    wasm_bindgen::__rt::link_mem_intrinsics();
                    inform(FUNCTION);
                    inform(0);
                    inform(0u32);
                    <Option<Vec<String>> as WasmDescribe>::describe();
                    <Option<Vec<String>> as WasmDescribe>::describe();
                }
            };
            #[cfg(target_arch = "wasm32")]
            #[automatically_derived]
            const _: () = {
                static _INCLUDED_FILES: &[&str] = &[];
                #[link_section = "__wasm_bindgen_unstable"]
                pub static _GENERATED: [u8; 136usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}R\x00\x00\x00\x01\x01\x08Template\x00\x00\x00\x00\x0bparseErrors\x01\x01\x00\x01\x00\x01\x0bparseErrors\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
            };
            self.parse_errors.clone()
        }
        pub fn text(&self) -> String {
            #[automatically_derived]
            const _: () = {
                #[export_name = "template_text"]
                pub unsafe extern "C" fn __wasm_bindgen_generated_Template_text(
                    me: u32,
                ) -> wasm_bindgen::convert::WasmRet<
                    <String as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
                > {
                    let _ret = {
                        let me = unsafe {
                            <Template as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                me,
                            )
                        };
                        let me = &*me;
                        let _ret = me.text();
                        _ret
                    };
                    <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                        .into()
                }
            };
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            #[automatically_derived]
            const _: () = {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C" fn __wbindgen_describe_template_text() {
                    use wasm_bindgen::describe::*;
                    wasm_bindgen::__rt::link_mem_intrinsics();
                    inform(FUNCTION);
                    inform(0);
                    inform(0u32);
                    <String as WasmDescribe>::describe();
                    <String as WasmDescribe>::describe();
                }
            };
            #[cfg(target_arch = "wasm32")]
            #[automatically_derived]
            const _: () = {
                static _INCLUDED_FILES: &[&str] = &[];
                #[link_section = "__wasm_bindgen_unstable"]
                pub static _GENERATED: [u8; 122usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}D\x00\x00\x00\x01\x01\x08Template\x00\x00\x00\x00\x04text\x01\x01\x00\x01\x00\x01\x04text\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
            };
            self.text.clone()
        }
        pub fn slots(&self) -> Vec<String> {
            #[automatically_derived]
            const _: () = {
                #[export_name = "template_slots"]
                pub unsafe extern "C" fn __wasm_bindgen_generated_Template_slots(
                    me: u32,
                ) -> wasm_bindgen::convert::WasmRet<
                    <Vec<String> as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
                > {
                    let _ret = {
                        let me = unsafe {
                            <Template as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                                me,
                            )
                        };
                        let me = &*me;
                        let _ret = me.slots();
                        _ret
                    };
                    <Vec<
                        String,
                    > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                        .into()
                }
            };
            #[cfg(
                all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                )
            )]
            #[automatically_derived]
            const _: () = {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C" fn __wbindgen_describe_template_slots() {
                    use wasm_bindgen::describe::*;
                    wasm_bindgen::__rt::link_mem_intrinsics();
                    inform(FUNCTION);
                    inform(0);
                    inform(0u32);
                    <Vec<String> as WasmDescribe>::describe();
                    <Vec<String> as WasmDescribe>::describe();
                }
            };
            #[cfg(target_arch = "wasm32")]
            #[automatically_derived]
            const _: () = {
                static _INCLUDED_FILES: &[&str] = &[];
                #[link_section = "__wasm_bindgen_unstable"]
                pub static _GENERATED: [u8; 124usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}F\x00\x00\x00\x01\x01\x08Template\x00\x00\x00\x00\x05slots\x01\x01\x00\x01\x00\x01\x05slots\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
            };
            self.slots.clone()
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 91usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}%\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
}
mod schema_and_entities {
    use std::str::FromStr;
    use cedar_policy::{Schema, Entities};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;
    use serde::{Serialize, Deserialize};
    #[serde(tag = "success")]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    /// struct that defines the result for the syntax validation function
    pub enum ParseResult {
        #[serde(rename = "true")]
        /// represents successful syntax validation
        Success,
        #[serde(rename = "false")]
        /// represents a syntax error and encloses a vector of the errors
        SyntaxError { errors: Vec<String> },
    }
    #[automatically_derived]
    const _: () = {
        extern crate serde as _serde;
        use tsify::Tsify;
        use wasm_bindgen::{
            convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
            describe::WasmDescribe, prelude::*,
        };
        #[automatically_derived]
        ///
        #[repr(transparent)]
        pub struct JsType {
            obj: wasm_bindgen::JsValue,
        }
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::convert::TryFromJsValue;
            use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
            use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
            use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
            use wasm_bindgen::describe::WasmDescribe;
            use wasm_bindgen::{JsValue, JsCast, JsObject};
            use wasm_bindgen::__rt::core;
            impl WasmDescribe for JsType {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(NAMED_EXTERNREF);
                    inform(11u32);
                    inform(80u32);
                    inform(97u32);
                    inform(114u32);
                    inform(115u32);
                    inform(101u32);
                    inform(82u32);
                    inform(101u32);
                    inform(115u32);
                    inform(117u32);
                    inform(108u32);
                    inform(116u32);
                }
            }
            impl IntoWasmAbi for JsType {
                type Abi = <JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    self.obj.into_abi()
                }
            }
            impl OptionIntoWasmAbi for JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl<'a> OptionIntoWasmAbi for &'a JsType {
                #[inline]
                fn none() -> Self::Abi {
                    0
                }
            }
            impl FromWasmAbi for JsType {
                type Abi = <JsValue as FromWasmAbi>::Abi;
                #[inline]
                unsafe fn from_abi(js: Self::Abi) -> Self {
                    JsType {
                        obj: JsValue::from_abi(js).into(),
                    }
                }
            }
            impl OptionFromWasmAbi for JsType {
                #[inline]
                fn is_none(abi: &Self::Abi) -> bool {
                    *abi == 0
                }
            }
            impl<'a> IntoWasmAbi for &'a JsType {
                type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
                #[inline]
                fn into_abi(self) -> Self::Abi {
                    (&self.obj).into_abi()
                }
            }
            impl RefFromWasmAbi for JsType {
                type Abi = <JsValue as RefFromWasmAbi>::Abi;
                type Anchor = core::mem::ManuallyDrop<JsType>;
                #[inline]
                unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                    core::mem::ManuallyDrop::new(JsType {
                        obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                    })
                }
            }
            impl LongRefFromWasmAbi for JsType {
                type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
                type Anchor = JsType;
                #[inline]
                unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                    JsType { obj: tmp.into() }
                }
            }
            impl From<JsValue> for JsType {
                #[inline]
                fn from(obj: JsValue) -> JsType {
                    JsType { obj: obj.into() }
                }
            }
            impl AsRef<JsValue> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsValue {
                    self.obj.as_ref()
                }
            }
            impl AsRef<JsType> for JsType {
                #[inline]
                fn as_ref(&self) -> &JsType {
                    self
                }
            }
            impl From<JsType> for JsValue {
                #[inline]
                fn from(obj: JsType) -> JsValue {
                    obj.obj.into()
                }
            }
            impl JsCast for JsType {
                fn instanceof(val: &JsValue) -> bool {
                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(
                        all(
                            target_arch = "wasm32",
                            not(any(target_os = "emscripten", target_os = "wasi"))
                        )
                    )]
                    extern "C" {
                        fn __wbg_instanceof_JsType_50645fbbdb6fda8b(val: u32) -> u32;
                    }
                    unsafe {
                        let idx = val.into_abi();
                        __wbg_instanceof_JsType_50645fbbdb6fda8b(idx) != 0
                    }
                }
                #[inline]
                fn unchecked_from_js(val: JsValue) -> Self {
                    JsType { obj: val.into() }
                }
                #[inline]
                fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                    unsafe { &*(val as *const JsValue as *const JsType) }
                }
            }
            impl JsObject for JsType {}
        };
        #[automatically_derived]
        impl core::ops::Deref for JsType {
            type Target = wasm_bindgen::JsValue;
            #[inline]
            fn deref(&self) -> &wasm_bindgen::JsValue {
                &self.obj
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 143usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}Y\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_50645fbbdb6fda8b\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl Tsify for ParseResult {
            type JsType = JsType;
            const DECL: &'static str = "export type ParseResult = { success: \"true\" } | { success: \"false\"; errors: string[] };";
        }
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 179usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}}\x00\x00\x00\x00\x00\x00\x00\x01Wexport type ParseResult = { success: \"true\" } | { success: \"false\"; errors: string[] };\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
        };
        impl WasmDescribe for ParseResult {
            #[inline]
            fn describe() {
                <Self as Tsify>::JsType::describe()
            }
        }
        impl IntoWasmAbi for ParseResult
        where
            Self: _serde::Serialize,
        {
            type Abi = <JsType as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.into_js().unwrap_throw().into_abi()
            }
        }
        impl OptionIntoWasmAbi for ParseResult
        where
            Self: _serde::Serialize,
        {
            #[inline]
            fn none() -> Self::Abi {
                <JsType as OptionIntoWasmAbi>::none()
            }
        }
        impl FromWasmAbi for ParseResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            type Abi = <JsType as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                let result = Self::from_js(&JsType::from_abi(js));
                if let Err(err) = result {
                    wasm_bindgen::throw_str(err.to_string().as_ref());
                }
                result.unwrap_throw()
            }
        }
        impl OptionFromWasmAbi for ParseResult
        where
            Self: _serde::de::DeserializeOwned,
        {
            #[inline]
            fn is_none(js: &Self::Abi) -> bool {
                <JsType as OptionFromWasmAbi>::is_none(js)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for ParseResult {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ParseResult::Success => ::core::fmt::Formatter::write_str(f, "Success"),
                ParseResult::SyntaxError { errors: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SyntaxError",
                        "errors",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ParseResult {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ParseResult::Success => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "ParseResult",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "success",
                            "true",
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    ParseResult::SyntaxError { ref errors } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "ParseResult",
                            0 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "success",
                            "false",
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "errors",
                            errors,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ParseResult {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "true" => _serde::__private::Ok(__Field::__field0),
                            "false" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"true" => _serde::__private::Ok(__Field::__field0),
                            b"false" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["true", "false"];
                let (__tag, __content) = _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::__private::de::TaggedContentVisitor::<
                        __Field,
                    >::new("success", "internally tagged enum ParseResult"),
                )?;
                let __deserializer = _serde::__private::de::ContentDeserializer::<
                    __D::Error,
                >::new(__content);
                match __tag {
                    __Field::__field0 => {
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                "ParseResult",
                                "Success",
                            ),
                        )?;
                        _serde::__private::Ok(ParseResult::Success)
                    }
                    __Field::__field1 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "errors" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"errors" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<ParseResult>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = ParseResult;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant ParseResult::SyntaxError",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Vec<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant ParseResult::SyntaxError with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(ParseResult::SyntaxError {
                                    errors: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("errors"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("errors")?
                                    }
                                };
                                _serde::__private::Ok(ParseResult::SyntaxError {
                                    errors: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["errors"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<ParseResult>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        }
    };
    #[allow(dead_code)]
    pub fn parse_schema(input_schema: &str) -> ParseResult {
        match Schema::from_str(input_schema) {
            Ok(_schema) => ParseResult::Success,
            Err(err) => {
                ParseResult::SyntaxError {
                    errors: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([err.to_string()]),
                    ),
                }
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "parseSchema"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_parseSchema(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <ParseResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let _ret = parse_schema(arg0);
                _ret
            };
            <ParseResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_parseSchema() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <ParseResult as WasmDescribe>::describe();
            <ParseResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 128usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}J\x00\x00\x00\x01\x00\x00\x00\x01\x0cinput_schema\x00\x0bparseSchema\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
    #[allow(dead_code)]
    pub fn parse_entities(entities_str: &str, schema_str: &str) -> ParseResult {
        let parsed_schema = match Schema::from_str(schema_str) {
            Ok(schema) => schema,
            Err(err) => {
                return ParseResult::SyntaxError {
                    errors: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([err.to_string()]),
                    ),
                };
            }
        };
        match Entities::from_json_str(entities_str, Some(&parsed_schema)) {
            Ok(_entities) => ParseResult::Success,
            Err(err) => {
                ParseResult::SyntaxError {
                    errors: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([err.to_string()]),
                    ),
                }
            }
        }
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "parseEntities"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_parseEntities(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <ParseResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let arg1 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg1_1,
                            arg1_2,
                            arg1_3,
                            arg1_4,
                        ),
                    )
                };
                let arg1 = &*arg1;
                let _ret = parse_entities(arg0, arg1);
                _ret
            };
            <ParseResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_parseEntities() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(2u32);
            <&str as WasmDescribe>::describe();
            <&str as WasmDescribe>::describe();
            <ParseResult as WasmDescribe>::describe();
            <ParseResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 141usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}W\x00\x00\x00\x01\x00\x00\x00\x02\x0centities_str\nschema_str\x00\rparseEntities\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
}
mod validator {
    use cedar_policy::frontend::{validate::json_validate, utils::InterfaceResult};
    use wasm_bindgen::prelude::*;
    #[allow(dead_code)]
    pub fn validate_semantics(input: &str) -> InterfaceResult {
        json_validate(input)
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "validateSemantics"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_validateSemantics(
            arg0_1: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <InterfaceResult as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                        <<str as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let arg0 = &*arg0;
                let _ret = validate_semantics(arg0);
                _ret
            };
            <InterfaceResult as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                .into()
        }
    };
    #[cfg(
        all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_validateSemantics() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <InterfaceResult as WasmDescribe>::describe();
            <InterfaceResult as WasmDescribe>::describe();
        }
    };
    #[cfg(target_arch = "wasm32")]
    #[automatically_derived]
    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 127usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}I\x00\x00\x00\x01\x00\x00\x00\x01\x05input\x00\x11validateSemantics\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bcedar-wasm-666de6321189be5c\x00\x00";
    };
}

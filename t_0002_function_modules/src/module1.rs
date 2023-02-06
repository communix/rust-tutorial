pub mod sub_module1;

// pub can be used for the struct or enum in the module
pub struct Module1Struct {
    pub module_name:String, // public element.
    version:u32,            // private element.
    enum_value: SubEnum,
}
impl Module1Struct {
    fn new(name:String) -> Module1Struct {
        Module1Struct {
            module_name: String::from(name),
            version: 1,
            enum_value: crate::module1::SubEnum::SubEnumOne,
        }
    }
}
pub enum SubEnum {
    SubEnumOne,
    SubEnumTwo,
}
pub struct GenModel{

}

pub struct GenInstance{

}

pub trait Model{
    fn new() -> Self;
    fn get_model_name(&self) -> String;
    fn get_model_type(&self) -> String;
    fn get_model_parameter(&self) -> String;
    fn get_model_value(&self) -> String;
}

pub trait Instance{
    fn new() -> Self;
    fn get_instance_name(&self) -> String;
    fn get_instance_model_name(&self) -> String;
    fn get_instance_parameter(&self) -> String;
    fn get_instance_value(&self) -> String;
}
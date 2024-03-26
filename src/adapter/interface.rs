
pub trait RSAEncrytion {
    fn new(bits : usize) -> Self;

    fn public_key(&self,private_key : String) -> Result<(String,String),&'static str>;

    fn private_key(&self) -> Result<String,&'static str>;

    fn encrypt(&self,message : String , public_key : String) -> Result<Vec<u8>,&'static str>;

    fn decrypt(&self,encrypted_message : Vec<u8> , private_key : String) -> Result<String,&'static str>;
}
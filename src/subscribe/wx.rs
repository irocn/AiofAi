use std::collections::BTreeMap;




pub struct WxPayClient{
    appid: String,
    mchid: String,
    apikey: String,
    notify_url: String
}

impl WxPayClient {
    fn new() -> Self{
        WxPayClient{
            appid: "wxf45f8284e2b1b03d".to_owned(),
            mchid: "1410859902".to_owned(),
            apikey: "B24EB5111E5CCC9BA7D5C7AED1C8951B".to_owned(),
            notify_url: "".to_owned()
        }
    }

    fn sign(){

    }

    fn create_order(self:&Self, ){
        let mut btree_order_req = BTreeMap::new();
        let mut str_order_req:String = "{".to_string();
      // create sign code
      for &(k,v) in [
          ("appid",       self.appid),
          ("mch_id",      self.mchid),
          ("out_trade_no", generate(32, "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPKRSTUVWXYZ")),
          ("spbill_create_ip", &self.spbill_create_ip),
          ("nonce_str",    generate(32, "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPKRSTUVWXYZ")),
          ("total_fee",    &self.total_fee.to_string()),
          ("trade_type",   &self.trade_type),
          ("notify_url",   self.notify_url),
          ("body",        &self.body)
      ].iter(){
          btree_order_req.insert(k.to_string(), v.to_string());
          str_order_req.push_str("\"");
          str_order_req.push_str(&k.to_string());
          str_order_req.push_str("\"");
          str_order_req.push_str(":");
          str_order_req.push_str("\"");
          str_order_req.push_str(&v.to_string());
          str_order_req.push_str("\"");
          str_order_req.push_str(",");
      }

     let _sign = WechatClient::generate_sign(&btree_order_req, &APIKEY.to_string());
     str_order_req.push_str("\"");
     str_order_req.push_str("sign");
     str_order_req.push_str("\"");
     str_order_req.push_str(":");
     str_order_req.push_str("\"");
     str_order_req.push_str(&_sign);
     str_order_req.push_str("\"");
     str_order_req.push_str("}");

     // insert sign code to btreemap 
     btree_order_req.insert("sign".to_string(), _sign);
     info!("the str_order_req is:{}", str_order_req);



     // create xml request
     let xml_order_req_str = WechatClient::to_xml_str(&btree_order_req);

     if self.trade_type == "APP" {
       return (self.out_trade_no.clone(), str_order_req);
     }
     (self.out_trade_no.clone(), xml_order_req_str)
    }

    fn create_qrcode(self:&Self){
        
    }
}

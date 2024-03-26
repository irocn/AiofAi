
pub mod wx;
pub mod zfb;


/*
static APPID: &str = "wxf45f8284e2b1b03d";
static MCHID: &str = "1410859902";
static APIKEY: &str = "B24EB5111E5CCC9BA7D5C7AED1C8951B";
static NOTIFY_URL: &str = "https://edux.dev/paycallback";       //todo: this is wx payment callback api
pub static EDUX_USERS_INDEX: &str = "edux_users";
pub static EDUX_PAYMENT_INDEX: &str = "edux_payment";

pub fn create_order_req(&self) -> (String, String) {
          let mut btree_order_req = BTreeMap::new();
          let mut str_order_req:String = "{".to_string();
        // create sign code
        for &(k,v) in [
            ("appid",       APPID),
            ("mch_id",      MCHID),
            ("out_trade_no", &self.out_trade_no),
            ("spbill_create_ip", &self.spbill_create_ip),
            ("nonce_str",    &self.nonce_str),
            ("total_fee",    &self.total_fee.to_string()),
            ("trade_type",   &self.trade_type),
            ("notify_url",   NOTIFY_URL),
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

    let (order_no, order_req) = WechatClient::create_order_req(&_client);

static WECHAT_UNIFILEDORDER_URL: &str = "https://api.mch.weixin.qq.com/pay/unifiedorder";
  let client = reqwest::Client::new();
  let resp =  client.post(WECHAT_UNIFILEDORDER_URL)
      .header("Content-Type", "application/xml")
      .body(order_req.clone())
      .send()
      .await
      .unwrap()
      .text()
      .await;
*/
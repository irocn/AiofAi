import { AfterViewInit, Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { WxLogin } from '../../assets/js/wxLogin';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent implements OnInit, AfterViewInit{

  inputValue: string = '';

  public getQRcode = () => {

  }

  ngOnInit(): void {
  }

  ngAfterViewInit(): void {
    var obj = new WxLogin({
      self_redirect:false,
      id:"wxqrcode", 
      appid: "wx5fb17a2c6d3a2c06", 
      scope: "snsapi_login", 
      redirect_uri: "http%3A%2F%2Fwx.iedux.pro%2Flogin%2F",
      state: "",
      style: "",
      href: "data:text/css;base64,LndycF9jb2Rlew0KICBtYXJnaW4tdG9wOiA2cHggIWltcG9ydGFudDsNCn0="
  });
  }
}

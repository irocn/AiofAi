interface WxLoginOptions {
    appid: string;
    scope: string;
    redirect_uri: string;
    state: string;
    self_redirect?: boolean;
    styletype?: string;
    sizetype?: string;
    bgcolor?: string;
    rst?: string;
    style?: string;
    href?: string;
    lang?: string;
    id: string;
  }
  
  export class WxLogin {
    constructor(options: WxLoginOptions) {
      let r = "default";
      if (options.self_redirect === true) {
        r = "true";
      } else if (options.self_redirect === false) {
        r = "false";
      }
      const n = document.createElement("iframe");
      let i =
        "https://open.weixin.qq.com/connect/qrconnect?appid=" +
        options.appid +
        "&scope=" +
        options.scope +
        "&redirect_uri=" +
        options.redirect_uri +
        "&state=" +
        options.state +
        "&login_type=jssdk&self_redirect=" +
        r +
        "&styletype=" +
        (options.styletype || "") +
        "&sizetype=" +
        (options.sizetype || "") +
        "&bgcolor=" +
        (options.bgcolor || "") +
        "&rst=" +
        (options.rst || "");
      i += options.style ? "&style=" + options.style : "";
      i += options.href ? "&href=" + options.href : "";
      i += options.lang === "en" ? "&lang=en" : "";
      n.src = i;
      n.frameBorder = "0";
      n.scrolling = "no";
      n.width = "300px";
      n.height = "400px";
      const s = document.getElementById(options.id);
      if (s) {
        s.innerHTML = "";
        s.appendChild(n);
      }
    }
  }
  
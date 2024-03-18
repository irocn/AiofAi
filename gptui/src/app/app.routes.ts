import { Routes } from '@angular/router';
import { HelpmeComponent } from './helpme/helpme.component';
import { LoginLogoutComponent } from './login-logout/login-logout.component';
import { SettingComponent } from './setting/setting.component';
import { MsgbodyComponent } from './msgbody/msgbody.component';
export const routes: Routes =  [
    { path: '', component: MsgbodyComponent},
    { path: 'help', component: HelpmeComponent },
    { path: 'settings', component: SettingComponent },
    { path: 'login', component: LoginLogoutComponent },
    { path: 'logout', component: LoginLogoutComponent},
    { path: 'msgbox', component: MsgbodyComponent}
    // Add more routes as needed
  ];
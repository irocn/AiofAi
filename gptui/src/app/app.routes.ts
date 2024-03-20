import { Routes } from '@angular/router';
import { HelpmeComponent } from './helpme/helpme.component';
import { SettingComponent } from './setting/setting.component';
import { MsgbodyComponent } from './msgbody/msgbody.component';
export const routes: Routes =  [
    { path: '', component: MsgbodyComponent},
    { path: 'help', component: HelpmeComponent },
    { path: 'settings', component: SettingComponent },
    { path: 'msgbox', component: MsgbodyComponent}
    // Add more routes as needed
  ];
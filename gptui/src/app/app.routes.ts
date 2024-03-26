import { Routes } from '@angular/router';
import { HelpmeComponent } from './helpme/helpme.component';
import { SettingComponent } from './setting/setting.component';
import { MsgbodyComponent } from './msgbody/msgbody.component';
import { LoginComponent } from './login/login.component';
import { BodycontainerComponent } from './bodycontainer/bodycontainer.component';
import { SubscribeComponent } from './subscribe/subscribe.component';
import { InputmsgboxComponent } from './inputmsgbox/inputmsgbox.component';

export const routes: Routes =  [
    { path: '', component: BodycontainerComponent},
    { path: 'bodybox', component: BodycontainerComponent},
    { path: 'help', component: HelpmeComponent },
    { path: 'settings', component: SettingComponent },
    { path: 'chat', component: MsgbodyComponent},
    { path: 'login', component: LoginComponent},
    { path: 'logout', component: LoginComponent},
    { path: 'subscribe', component: SubscribeComponent},
    { path: 'inputbox', component: InputmsgboxComponent}
  ];

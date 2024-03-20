import { AfterViewInit, Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { MsgbodyComponent } from "./msgbody/msgbody.component";
import { InputmsgboxComponent } from "./inputmsgbox/inputmsgbox.component";
import { BodycontainerComponent } from "./bodycontainer/bodycontainer.component";
import { LoginComponent } from "./login/login.component";
import { LogoutComponent } from "./logout/logout.component";
import { SettingComponent } from "./setting/setting.component";
import { HelpmeComponent } from "./helpme/helpme.component";
import { SubscribeComponent } from "./subscribe/subscribe.component";
import { WebSocketService } from "./websocket.service";
import { EventsService } from './events.service';
import { CommonModule } from '@angular/common';
import { ApiService } from './api.service';
import { HttpClientModule } from '@angular/common/http';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
            RouterOutlet, 
            CommonModule, 
            BodycontainerComponent, 
            SidebarComponent, 
            MsgbodyComponent, 
            InputmsgboxComponent, 
            LoginComponent, 
            LogoutComponent, 
            SettingComponent,
            HelpmeComponent,
            SubscribeComponent,
            HttpClientModule
          ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  providers: [WebSocketService, ApiService],
})
export class AppComponent{
  
  toDisplayComponent:string = 'siteinfo';

  constructor(private eventService: EventsService,){
    this.eventService.getEvent().subscribe((event: any) => {
      switch(event){
        case 'siteinfo':
          this.toDisplayComponent = 'siteinfo';
        break;
        case 'login':
          this.toDisplayComponent = 'login';
        break;
        case 'logout':
          this.toDisplayComponent = 'logout';
        break;
        case 'setting':
          this.toDisplayComponent = 'setting';
        break;
        case 'help':
          this.toDisplayComponent = 'help';
        break;
        case 'chat':
          this.toDisplayComponent = 'chat';
        break;    
        case 'subscribe':
            this.toDisplayComponent = 'subscribe';
        break;    
      }
    });
  }


  title = 'Hello Gpt!';
}


import { AfterViewInit, Component } from '@angular/core';
import { Router, RouterOutlet } from '@angular/router';
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
            HttpClientModule,
          ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  providers: [WebSocketService, ApiService, ],
})
export class AppComponent{
  
  isShowInputBox = false;
  isCloseSideBar = false;

  constructor(private eventService: EventsService, private router: Router){
    this.eventService.getEvent().subscribe((event: any) => {
      switch(event){
        case 'bodybox':
          this.isShowInputBox = false;
          this.router.navigate(['/bodybox']);
        break;
        case 'login':
          this.isShowInputBox = false;
          this.router.navigate(['/login']);
        break;
        case 'logout':
          this.isShowInputBox = false;
          this.router.navigate(['/logout']);
        break;
        case 'setting':
          this.isShowInputBox = false;
          this.router.navigate(['/setting']);
        break;
        case 'help':
          this.isShowInputBox = false;
          this.router.navigate(['/help']);
        break;
        case 'chat':
          this.isShowInputBox = true;
          this.router.navigate(['/chat']);
        break;    
        case 'subscribe':
          this.isShowInputBox = false;
          this.router.navigate(['/subscribe']);
        break;    
        case 'closesidebar':
          this.isCloseSideBar = !this.isCloseSideBar;
          console.log("closesidebar");
        break;
      }
    });
  }

  title = 'Hello Gpt!';
}


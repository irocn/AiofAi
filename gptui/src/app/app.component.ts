import { AfterViewInit, Component } from '@angular/core';
import { Router, RouterOutlet } from '@angular/router';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { BodycontainerComponent } from "./bodycontainer/bodycontainer.component";
import { LoginComponent } from "./login/login.component";
import { LogoutComponent } from "./logout/logout.component";
import { SettingComponent } from "./setting/setting.component";
import { HelpmeComponent } from "./helpme/helpme.component";
import { SubscribeComponent } from "./subscribe/subscribe.component";
import { ChatComponent } from "./chat/chat.component";
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
            LoginComponent, 
            LogoutComponent, 
            SettingComponent,
            HelpmeComponent,
            SubscribeComponent,
            ChatComponent,
            HttpClientModule,
          ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  providers: [WebSocketService, ApiService, ],
})
export class AppComponent{
  
  isShowInputBox = false;

  constructor(private eventService: EventsService, private router: Router){
    this.eventService.getEvent().subscribe((event: any) => {
      switch(event){
        case 'bodybox':
          this.router.navigate(['/bodybox']);
        break;
        case 'login':
          this.router.navigate(['/login']);
        break;
        case 'logout':
          this.router.navigate(['/logout']);
        break;
        case 'setting':
          this.router.navigate(['/setting']);
        break;
        case 'help':
          this.router.navigate(['/help']);
        break;
        case 'chat':
          this.router.navigate(['/chat']);
        break;    
        case 'subscribe':
          this.router.navigate(['/subscribe']);
        break;    
      }
    });
  }

  title = 'Hello Gpt!';
  isSidebarOpen: boolean = true;

  toggleSidebar() {
    this.isSidebarOpen = !this.isSidebarOpen;
  }
}


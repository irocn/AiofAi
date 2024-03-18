import { AfterViewInit, Component, ComponentFactoryResolver, ViewChild, ViewContainerRef } from '@angular/core';
import { HelpmeComponent } from "../helpme/helpme.component";
import { SettingComponent } from "../setting/setting.component";
import { LoginLogoutComponent } from "../login-logout/login-logout.component";
import { MsgbodyComponent } from "../msgbody/msgbody.component";
import { RouterOutlet } from '@angular/router';
import { EventsService } from '../events.service';

@Component({
  selector: 'app-bodycontainer',
  standalone: true,
  imports: [RouterOutlet,HelpmeComponent, SettingComponent, LoginLogoutComponent, MsgbodyComponent],
  templateUrl: './bodycontainer.component.html',
  styleUrl: './bodycontainer.component.scss'
})
export class BodycontainerComponent{
  receivedEvent: any;

  constructor(private eventService: EventsService, private componentFactoryResolver: ComponentFactoryResolver) {
    this.eventService.getEvent().subscribe((event: any) => {
      this.container.clear();
      switch(event){
        case 'msgbox':
          const msgbox = this.componentFactoryResolver.resolveComponentFactory(MsgbodyComponent);
          this.container.createComponent(msgbox); //todo: replace the deprecated method
          break;
        case 'login':
          const loginlogout = this.componentFactoryResolver.resolveComponentFactory(LoginLogoutComponent);
          this.container.createComponent(loginlogout);
          break;  
        case "settings":
          const settings = this.componentFactoryResolver.resolveComponentFactory(SettingComponent);
          this.container.createComponent(settings);
          break;
        case "help":
          const _help = this.componentFactoryResolver.resolveComponentFactory(HelpmeComponent);
          this.container.createComponent(_help);
          break;
        default:
          console.log(event);
          break;  
      }
    });
  }

  @ViewChild('container', { read: ViewContainerRef })
  container!: ViewContainerRef;
}

import { AfterViewInit, Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { MsgbodyComponent } from "./msgbody/msgbody.component";
import { InputmsgboxComponent } from "./inputmsgbox/inputmsgbox.component";
import { BodycontainerComponent } from "./bodycontainer/bodycontainer.component"
import { WebSocketService } from "./websocket.service";
import { EventsService } from './events.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, BodycontainerComponent, SidebarComponent, MsgbodyComponent, InputmsgboxComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  providers: [WebSocketService],
})
export class AppComponent{
  title = 'Hello Gpt!';
}


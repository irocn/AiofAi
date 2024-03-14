import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SidebarComponent } from "./sidebar/sidebar.component";
import { MsgbodyComponent } from "./msgbody/msgbody.component";
import { InputmsgboxComponent } from "./inputmsgbox/inputmsgbox.component";
import { WebSocketService } from "./websocket.service";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, SidebarComponent, MsgbodyComponent, InputmsgboxComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  providers: [WebSocketService],
})
export class AppComponent {
  title = 'Hello Gpt!';

  message: string = '';

  constructor(private websocketService: WebSocketService) {}
/*
  sendMessage() {
    this.websocketService.sendMessage(this.message);
    this.message = '';
  }

  ngOnInit() {
    console.log("websocket to connect server");
    this.websocketService.connect();
  }

  ngOnDestroy() {
    this.websocketService.disconnect();
  }
  */
}


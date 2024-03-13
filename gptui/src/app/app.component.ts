import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

import { WebsocketService } from "./websocket.service";
import { ChatService } from "./chat.service";
import { SidebarComponent } from "./sidebar/sidebar.component";
import { MsgbodyComponent } from "./msgbody/msgbody.component";
import { InputmsgboxComponent } from "./inputmsgbox/inputmsgbox.component";


@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, SidebarComponent, MsgbodyComponent, InputmsgboxComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  providers: [WebsocketService, ChatService],
})
export class AppComponent {
  title = 'Hello Gpt!';

  constructor(private chatService: ChatService) {
    chatService.messages.subscribe(msg => {
      console.log("Response from websocket: " + msg);
    });
  }

  private message = {
    author: "tutorialedge",
    msg: "hello"
  };

  sendMsg() {
    console.log("new message from client to websocket: ", this.message);
    this.chatService.messages.next(this.message);
    this.message.msg = "";
  }
  
}


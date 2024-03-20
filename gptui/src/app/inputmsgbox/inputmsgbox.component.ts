import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms'; 
import { CommonModule } from '@angular/common';
import { WebSocketService } from "../websocket.service";
import { EventsService } from '../events.service';

@Component({
  selector: 'app-inputmsgbox',
  standalone: true,
  imports: [FormsModule,CommonModule],
  templateUrl: './inputmsgbox.component.html',
  styleUrl: './inputmsgbox.component.scss'
})
export class InputmsgboxComponent {
  message: String = '';
  startChat: boolean = false;

  constructor(private websocket: WebSocketService, private eventService: EventsService) {}

  sendMessage() {
    if ( this.startChat == false){
      this.eventService.sendEvent('chat');
      this.startChat = true;

    }
    this.websocket.sendMessage(this.message);
  }

}

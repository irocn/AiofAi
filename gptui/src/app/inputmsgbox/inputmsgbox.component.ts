import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms'; 
import { CommonModule } from '@angular/common';
import { WebSocketService } from "../websocket.service";

@Component({
  selector: 'app-inputmsgbox',
  standalone: true,
  imports: [FormsModule,CommonModule],
  templateUrl: './inputmsgbox.component.html',
  styleUrl: './inputmsgbox.component.scss'
})
export class InputmsgboxComponent {
  message: String = '';

  constructor(private websocket: WebSocketService) {}

  sendMessage() {
    this.websocket.sendMessage(this.message);
  }

}

import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { WebSocketService } from "../websocket.service";

@Component({
  selector: 'app-msgbody',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './msgbody.component.html',
  styleUrl: './msgbody.component.scss',
  providers: [],
})
export class MsgbodyComponent {
  isVisible: boolean = true;
  receivedMessage: any[] = [];

  constructor(private WebSocketService: WebSocketService) {}


  ngOnInit(): void {
    this.WebSocketService.getMessage().subscribe(
      message => {
        this.isVisible = false;
        this.receivedMessage.push(message);
      },
      error => console.error('Error receiving message:', error)
    );
  }

  decodemsg(msg:any){
    let _base64_decode:string =  atob(msg);

    if (_base64_decode.startsWith('data: [DONE]')){
      return;
    }

    if (_base64_decode.startsWith('data: {"type"')){
      return;
    }

    if (_base64_decode.startsWith('data: {"message"')){
      let _json = JSON.parse(_base64_decode.substring(5));
      let _content:string = _json["message"]["content"]["parts"];
      if ( _content.length != 0 ){
        return _content;
      }
    }
    return    
  }
}

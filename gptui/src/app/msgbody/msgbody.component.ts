import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { WebSocketService } from "../websocket.service";
import { provideMarkdown } from 'ngx-markdown';
import { MarkdownModule } from 'ngx-markdown';

@Component({
  selector: 'app-msgbody',
  standalone: true,
  imports: [CommonModule,MarkdownModule],
  templateUrl: './msgbody.component.html',
  styleUrl: './msgbody.component.scss',
  providers: [provideMarkdown()],
})
export class MsgbodyComponent {
  isVisible: boolean = true;

  // Conversion variable
  receivedMessages:{[conversation_id: string]:string} = {};
  markdown = ``;


  // Message
  title: string = "";

  constructor(private WebSocketService: WebSocketService) {}

  ngOnInit(): void {
    this.WebSocketService.getMessage().subscribe(
      message => {
        // When have message received to don't display the messagboard
        this.isVisible = false;

        // Handle the message received
        // 1. base64 decode
        let _body = atob(message.body);

        // 2. category the message
        if ( _body.startsWith("data: [DONE]")){
          
          // handle done

        }else{
          let _json = JSON.parse(_body.substring(5))
          if ( _json.type !== undefined ){
            
            // handle title created

          }else if ( _json.message !== undefined ) {

            console.log(`id: ${_json.message.id}`);
            console.log(`content: ${JSON.stringify(_json.message)}`);

            if ( _json.message.content.parts == ""){
              return;
            }

            if ( !_json.message.metadata.hasOwnProperty("parent_id") ){
              let _title = _json.message.content.parts[0];
              _title = _title.replace(/"/g, ' ');
              this.receivedMessages[_json.message.create_time] = "<h5><i class=\"bi bi-person-fill\"></i>" + _title + "</h5>";
              return;
            }

            for ( let item of _json.message.content.parts){
              this.receivedMessages[_json.message.create_time] = "<i class=\"bi bi-robot\"></i> "+item;
            }
          } 
        }
      },
      error => console.error('Error receiving message:', error)
    );
  }

  decodemsg(msg:any){
    if ( msg === undefined ){
      return;
    }
    
    let _base64_decode:string = atob(msg.body);

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
    return;
  }

  onReady(){}
  /*
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
  */
}

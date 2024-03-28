import { Component, ElementRef, ViewChild } from '@angular/core';
import { WebSocketService } from '../websocket.service';
import { EventsService } from '../events.service';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { MarkdownModule, provideMarkdown } from 'ngx-markdown';

@Component({
  selector: 'app-chat',
  standalone: true,
  imports: [FormsModule,CommonModule,MarkdownModule],
  templateUrl: './chat.component.html',
  styleUrl: './chat.component.scss',
  providers: [provideMarkdown()],
})
export class ChatComponent {
  title: string = '';
  message: String = '';
  markdown = ``;
  startChat: boolean = false;
  receivedMessages:{[conversation_id: string]:string} = {};
  @ViewChild('messageContainer') private messageContainer!: ElementRef;

  constructor(private websocket: WebSocketService, private eventService: EventsService) {}

  ngAfterViewInit(): void {
    this.eventService.sendEvent('msgbox');
    try {
      this.messageContainer.nativeElement.scrollTop = this.messageContainer.nativeElement.scrollHeight;
    } catch(err) { }
  }

  ngOnInit(): void {
    this.websocket.getMessage().subscribe(
      message => {
        // When have message received to don't display the messagboard

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

  sendMessage() {
    if ( this.startChat == false){
      this.eventService.sendEvent('chat');
      this.startChat = true;

    }
    this.websocket.sendMessage(this.message);
  }


}

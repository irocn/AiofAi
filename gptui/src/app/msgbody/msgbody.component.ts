import { AfterViewInit, Component, ElementRef, ViewChild} from '@angular/core';
import { CommonModule } from '@angular/common';
import { WebSocketService } from "../websocket.service";
import { provideMarkdown } from 'ngx-markdown';
import { MarkdownModule } from 'ngx-markdown';
import { EventsService } from '../events.service';


@Component({
  selector: 'app-msgbody',
  standalone: true,
  imports: [CommonModule,MarkdownModule],
  templateUrl: './msgbody.component.html',
  styleUrl: './msgbody.component.scss',
  providers: [provideMarkdown()],
})
export class MsgbodyComponent implements AfterViewInit{
  @ViewChild('messageContainer') private messageContainer!: ElementRef;


  // Conversion variable
  receivedMessages:{[conversation_id: string]:string} = {};
  markdown = ``;


  // Message
  title: string = "";
  constructor(private WebSocketService: WebSocketService, private eventService: EventsService) {}

  ngAfterViewInit(): void {
    this.eventService.sendEvent('msgbox');
    try {
      this.messageContainer.nativeElement.scrollTop = this.messageContainer.nativeElement.scrollHeight;
    } catch(err) { }
  }

  ngOnInit(): void {
    this.WebSocketService.getMessage().subscribe(
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
}

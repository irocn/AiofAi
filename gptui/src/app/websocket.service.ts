// websocket.service.ts
import { Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs/Rx';
import { WebSocketSubject, webSocket } from 'rxjs/webSocket';

@Injectable()
export class WebSocketService {

  private socket$: WebSocketSubject<any>;
  private messagesSubject: Subject<any> = new Subject<any>();

  constructor() {

    this.socket$ = webSocket('ws://localhost:4133/ws/haogle/'); // Replace with your WebSocket server URL
 
    this.socket$.subscribe(
      message => this.messagesSubject.next(message),
      error => console.error('WebSocket error:', error),
      () => console.log('WebSocket connection closed')
    );
  }

  sendMessage(message: any): void {
    this.socket$.next( String(message) );
  }

  getMessage(): Observable<any> {
    console.log("receive msg");
    return this.messagesSubject.asObservable();
  }
}



//import { Socket, SocketIoConfig } from 'ngx-socket-io';
//import io from 'socket.io-client';

/*
const config: SocketIoConfig = { 
  url: 'ws://db.irocn.com:4133', 
  options: {
      path: "/ws/user",
      reconnection: true,
      transports: ['websocket'],
      autoConnect: true
  } 
};
*/
/*
@Injectable({
  providedIn: 'root'
})
export class WebsocketService {

  private socket;

  constructor() { 
    this.socket = io('ws://db.irocn.com:4133');
  }

  sendMessage(message: string) {
    return this.socket.emit('message');
  }


  socket = new Socket(config)

  connect() {
    console.log('to connet .....');
    this.socket.connect();
  }

  sendMessage(message: string) {
    console.log(`send msg to server ${message}`);
    this.socket.emit(message);
  }

  onMessage() {
    return this.socket.fromEvent('message');
  }

  disconnect() {
    this.socket.disconnect();
  }
  
}
*/
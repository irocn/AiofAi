import { Injectable } from '@angular/core';
import { Subject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class EventsService {

  private eventSubject = new Subject<any>();

  sendEvent(event: any) {
    this.eventSubject.next(event);
  }

  getEvent() {
    return this.eventSubject.asObservable();
  }

}

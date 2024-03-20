import { Component } from '@angular/core';
import { EventsService } from "../events.service";

@Component({
  selector: 'app-subscribe',
  standalone: true,
  imports: [],
  templateUrl: './subscribe.component.html',
  styleUrl: './subscribe.component.scss'
})
export class SubscribeComponent {

  constructor(private eventService: EventsService) {}

  sendEvent(action:any) {
    this.eventService.sendEvent(action);
  }
}

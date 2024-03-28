import { Component, ViewChild, ViewContainerRef } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { EventsService } from '../events.service';

@Component({
  selector: 'app-bodycontainer',
  standalone: true,
  imports: [RouterOutlet],
  templateUrl: './bodycontainer.component.html',
  styleUrl: './bodycontainer.component.scss'
})
export class BodycontainerComponent{
  receivedEvent: any;

  constructor(private eventService: EventsService) {}

  @ViewChild('container', { read: ViewContainerRef })
  container!: ViewContainerRef;
}

import { AfterViewInit, Component } from '@angular/core';
import { NgClass, NgStyle } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { EventsService } from "../events.service";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [RouterOutlet, NgClass, NgStyle],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent implements AfterViewInit{
  isSidebarOpen = false;

  constructor(private eventService: EventsService) {}
  
  toggleSidebar() {
    this.isSidebarOpen = !this.isSidebarOpen;
  }

  sendEvent(action:any) {
    this.eventService.sendEvent(action);
  }

  ngAfterViewInit(): void {
    //this.eventService.sendEvent('msgbox');
  }
}

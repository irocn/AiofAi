import { AfterViewInit, Component, OnInit } from '@angular/core';
import { NgClass, NgStyle, NgFor } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { EventsService } from "../events.service";
import { ApiService } from "../api.service";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [RouterOutlet, NgClass, NgStyle, NgFor],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent implements OnInit, AfterViewInit{
  isSidebarOpen = true;
  isLogined = false;
  email = '';
  verify_code = '';

  chats: any [] = [];

  constructor(private eventService: EventsService, private api: ApiService) {}

  chats_req = {
    userid: "haogle"
  };

  ngOnInit(): void {    
    this.api.getChatRecordes(this.chats_req).subscribe(
      (data) => {
        console.log(data);
        this.chats = data;
      },
      (error) => {
        console.error('Error fetching users:', error);
      }
    );
  }
  
  toggleSidebar() {
    this.isSidebarOpen = !this.isSidebarOpen;
  }

  sendEvent(action:any) {
    this.eventService.sendEvent(action);
  }

  ngAfterViewInit(): void {
    //this.eventService.sendEvent('msgbox');
  }

  login(){
    const payload = {email: this.email, verify_code: this.verify_code};
    return this.api.login(payload).subscribe(
      response => {
        const token = response.data.token;
        localStorage.setItem('token', token); // save token to local storage
        return true;
      },
      error => {
        return false;
      }
    );
  }

  logout(){
    const _token = localStorage.getItem('token');
    const payload = {token:_token};

    this.api.logout(payload).subscribe(
      response => {
        console.log('Logout successful:', response);
      },
      error => {
        console.error('Logout failed:', error);
      }
    );
  }

  
}
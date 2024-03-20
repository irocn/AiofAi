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
  isSidebarOpen = false;
  chats: any [] = [];

  constructor(private eventService: EventsService, private http: ApiService) {}

  ngOnInit(): void {    
    this.http.getChatRecordes().subscribe(
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
}
/*
  private apiUrl = 'https://example.com/api/users'; // 假设服务端 API 的地址为 https://example.com/api/users

  constructor(private http: HttpClient) { }

  getUsers(): Observable<any[]> {
    return this.http.get<any[]>(this.apiUrl); // 发起 GET 请求获取用户数据
  }
  */
import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';

import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private apiUrl = 'http://localhost:4133/';
  private token = '';

  httpOptions = {
    headers: new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': `${this.token}`
    })
  };
  
  constructor(private api: HttpClient) {}

  getChatRecordes( payload: any ): Observable<any[]> {
    return this.api.post<any[]>(`${this.apiUrl}chats/`, payload , this.httpOptions);
  }

  login(payload:any):Observable<any>{
    return this.api.post<any>(`${this.apiUrl}login/`, payload, this.httpOptions);
  }

  logout(payload:any): Observable<any> {
    return this.api.post<any>(`${this.apiUrl}logout/`, null, this.httpOptions);
  }

  help(payload:any): Observable<any> {
    return this.api.post<any>(`${this.apiUrl}help/`, payload, this.httpOptions);
  }

  setting(payload:any): Observable<any> {
    return this.api.post<any>(`${this.apiUrl}setting/`, payload, this.httpOptions);
  }

  buyservice(payload:any): Observable<any> {
    return this.api.post<any>(`${this.apiUrl}buyservice/`, payload, this.httpOptions);
  }
}

import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MsgbodyComponent } from './msgbody.component';

describe('MsgbodyComponent', () => {
  let component: MsgbodyComponent;
  let fixture: ComponentFixture<MsgbodyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MsgbodyComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MsgbodyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

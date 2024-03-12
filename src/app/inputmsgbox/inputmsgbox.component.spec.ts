import { ComponentFixture, TestBed } from '@angular/core/testing';

import { InputmsgboxComponent } from './inputmsgbox.component';

describe('InputmsgboxComponent', () => {
  let component: InputmsgboxComponent;
  let fixture: ComponentFixture<InputmsgboxComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [InputmsgboxComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(InputmsgboxComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

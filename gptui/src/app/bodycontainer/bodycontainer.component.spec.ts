import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BodycontainerComponent } from './bodycontainer.component';

describe('BodycontainerComponent', () => {
  let component: BodycontainerComponent;
  let fixture: ComponentFixture<BodycontainerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BodycontainerComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(BodycontainerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

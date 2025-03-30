import { HttpClient } from '@angular/common/http';
import { Component, inject } from '@angular/core';
import { FormBuilder, ReactiveFormsModule } from '@angular/forms';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [ReactiveFormsModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  private apiUrl = '//localhost:8080';
  private httpClient = inject(HttpClient);
  private formBuilder = inject(FormBuilder);

  form = this.formBuilder.group({ name: '', password: '' });

  login() {
    this.httpClient.post(
      `${this.apiUrl}/login`,
      this.form.value,
      { withCredentials: true }
    ).subscribe({
      error: error => window.alert(error.error),
      complete: () => window.alert('login completed')
    });
  }

  logout() {
    this.httpClient.post(
      `${this.apiUrl}/logout`,
      null,
      { withCredentials: true }
    ).subscribe({
      error: error => window.alert(error.error),
      complete: () => window.alert('logout completed')
    });
  }
}

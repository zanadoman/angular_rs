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

  loginForm = this.formBuilder.group({ name: '', password: '' });
  registerForm = this.formBuilder.group({ name: '', password: '' });

  register() {
    this.httpClient.post(
      `${this.apiUrl}/register`,
      this.registerForm.value
    ).subscribe({
      error: error => window.alert(`${error.status}: ${error.error}`),
      complete: () => window.alert('Register completed.')
    });
  }

  login() {
    this.httpClient.post(
      `${this.apiUrl}/login`,
      this.loginForm.value,
      { withCredentials: true }
    ).subscribe({
      error: error => window.alert(`${error.status}: ${error.error}`),
      complete: () => window.alert('Login completed.')
    });
  }

  logout() {
    this.httpClient.post(
      `${this.apiUrl}/logout`,
      null,
      { withCredentials: true }
    ).subscribe({
      error: error => window.alert(`${error.status}: ${error.error}`),
      complete: () => window.alert('Logout completed.')
    });
  }
}

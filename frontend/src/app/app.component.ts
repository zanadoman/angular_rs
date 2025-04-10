import { HttpClient } from '@angular/common/http';
import { Component, inject } from '@angular/core';
import { FormBuilder, ReactiveFormsModule } from '@angular/forms';

@Component({
  selector: 'app-root',
  imports: [ReactiveFormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  private apiUrl = '//localhost:3000';
  private httpClient = inject(HttpClient);
  private formBuilder = inject(FormBuilder);

  registerForm = this.formBuilder.group({ name: '', password: '' });
  loginForm = this.formBuilder.group({ name: '', password: '' });

  register() {
    console.log('Register request started.');
    this.httpClient.post(
      `${this.apiUrl}/register`,
      this.registerForm.value
    ).subscribe({
      error: error => {
        console.log(`${error.error}\nRegister request failed.`);
        window.alert(`${error.status}: ${error.error}`);
      },
      complete: () => {
        console.log('Register request completed.');
        window.alert('Register request completed.');
      }
    });
  }

  login() {
    console.log('Login request started.');
    this.httpClient.post(
      `${this.apiUrl}/login`,
      this.loginForm.value,
      { withCredentials: true }
    ).subscribe({
      error: error => {
        console.log(`${error.error}\nLogin request failed.`);
        window.alert(`${error.status}: ${error.error}`);
      },
      complete: () => {
        console.log('Login request completed.');
        window.alert('Login request completed.');
      }
    });
  }

  logout() {
    console.log('Logout request started.');
    this.httpClient.post(
      `${this.apiUrl}/logout`,
      null,
      { withCredentials: true }
    ).subscribe({
      error: error => {
        console.log(`${error.error}\nLogout request failed.`);
        window.alert(`${error.status}: ${error.error}`);
      },
      complete: () => {
        console.log('Logout request completed.');
        window.alert('Logout request completed.');
      }
    });
  }
}

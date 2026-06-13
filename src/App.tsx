import { RouterProvider } from 'react-router-dom';
import { ThemeProvider } from 'next-themes';
import router from './router';
import './index.css';
import { Toaster } from "@/components/ui/sonner";
function App() {

  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
      <Toaster position='top-center'/>
      <RouterProvider router={router} />
    </ThemeProvider>
  );
}

export default App;

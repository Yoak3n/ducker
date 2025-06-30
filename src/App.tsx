import { RouterProvider } from 'react-router-dom';
import router from './router';
import './index.css';
import { Toaster } from "@/components/ui/sonner";
function App() {


  return (
    <div>
      <Toaster position='top-center'/>
      <RouterProvider router={router} />
    </div>
  );
}

export default App;

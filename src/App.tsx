import { useEffect } from 'react';
import { RouterProvider } from 'react-router-dom';
import router from './router';
import './index.css';
import { Toaster } from "@/components/ui/sonner";
import * as store from '@/store'
function App() {
  
  useEffect(()=>{
    store.clearAllErrors();
  },[])
  return (
    <>
      <Toaster position='top-center'/>
      <RouterProvider router={router} />
    </>
  );
}

export default App;

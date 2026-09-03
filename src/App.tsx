import { RouterProvider } from 'react-router-dom';
import { ThemeProvider } from 'next-themes';
import router from './router';
import './index.css';
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
function App() {

  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
      {/* Single provider at the root so adjacent tooltips can skip the delay */}
      <TooltipProvider delayDuration={150} skipDelayDuration={0}>
        <Toaster position='top-center' richColors />
        <RouterProvider router={router} />
      </TooltipProvider>
    </ThemeProvider>
  );
}

export default App;

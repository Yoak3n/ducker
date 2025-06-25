interface Props {
    position?: { x: number; y: number };
    menuItems?: MenuItem[];
    hideMenuCallback: (flag: boolean) => void;
}

interface MenuItem {
  label: string;
  action: () => void;
}



export default function Live2DContextMenu({ position = { x: 0, y: 0 },hideMenuCallback,menuItems}: Props ) {

  return (
    <div className="live2d-context-menu" style={{
            position: 'fixed',
            top: `${position.y}px`,
            left: `${position.x}px`,
            backgroundColor: '#fff',
            border: '1px solid #ccc',
            borderRadius: '4px',
            boxShadow: '0 2px 10px rgba(0,0,0,0.2)',
            padding: '5px 0',
            zIndex: 1000
          }}>
      {menuItems?.map((item, index) => (
            <div 
              key={index}
              onClick={(e) => {
                e.stopPropagation();
                item.action();
                hideMenuCallback(false);
              }}
              style={{
                padding: '8px 15px',
                cursor: 'pointer',
                // hover: {
                //   backgroundColor: '#f5f5f5'
                // }
              }}
              onMouseOver={(e) => {
                e.currentTarget.style.backgroundColor = '#f5f5f5';
              }}
              onMouseOut={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              {item.label}
            </div>
          ))}
        
    </div>
  );
} 
import React from 'react';
import { Button } from '@/components/ui/button';
import { Link } from 'react-router-dom';

const NotFound: React.FC = () => {
  return (
    <div className="not-found-page">
      <h1>404</h1>
      <h2>页面未找到</h2>
      <p>抱歉，您访问的页面不存在。</p>
      <Link to="/" className="back-home">
        返回首页
      </Link>
      <Button
        variant="outline"
        className="mt-4"
        onClick={() => window.location.reload()}
      >
        刷新页面
      </Button>
    </div>
  );
};

export default NotFound;
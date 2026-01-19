import { Tag } from 'antd';
import type { ChannelInfo } from '../types';

interface ChannelBadgeProps {
  channels: ChannelInfo[];
}

const CHANNEL_COLORS: Record<string, string> = {
  production: 'green',
  test: 'orange',
  development: 'blue',
};

export const ChannelBadge = ({ channels }: ChannelBadgeProps) => {
  if (channels.length === 0) {
    return <Tag color="default">Unpublished</Tag>;
  }

  return (
    <>
      {channels.map((channel) => {
        const color = CHANNEL_COLORS[channel.name.toLowerCase()] || 'default';
        const label = channel.name.charAt(0).toUpperCase() + channel.name.slice(1);
        return (
          <Tag key={channel.id} color={color} style={{ marginRight: 4 }}>
            {label}
          </Tag>
        );
      })}
    </>
  );
};

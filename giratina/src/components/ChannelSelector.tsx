import { Select } from 'antd';
import { useChannels } from '../hooks/useChannels';

interface ChannelSelectorProps {
  value?: number[];
  onChange: (value: number[]) => void;
  disabled?: boolean;
  placeholder?: string;
}

export const ChannelSelector = ({
  value,
  onChange,
  disabled,
  placeholder = 'Select channels',
}: ChannelSelectorProps) => {
  const { data: channels = [], isLoading } = useChannels();

  const options = channels.map((channel) => ({
    label: channel.name.charAt(0).toUpperCase() + channel.name.slice(1),
    value: channel.id,
  }));

  return (
    <Select
      mode="multiple"
      value={value}
      onChange={onChange}
      options={options}
      disabled={disabled}
      placeholder={placeholder}
      loading={isLoading}
      style={{ width: '100%' }}
    />
  );
};

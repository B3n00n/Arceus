import { useState } from 'react';
import {
  Typography,
  Button,
  Table,
  Space,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
  Modal,
  Form,
  Input,
  App,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  TagsOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { useChannels } from '../hooks/useChannels';
import {
  useCreateChannel,
  useUpdateChannel,
  useDeleteChannel,
} from '../hooks/useChannelMutations';
import type { ReleaseChannel } from '../types';

dayjs.extend(relativeTime);

const { Title } = Typography;
const { TextArea } = Input;

export const ChannelsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedChannel, setSelectedChannel] = useState<ReleaseChannel | undefined>();
  const [form] = Form.useForm();

  const { data: channels = [], isLoading } = useChannels();
  const { message } = App.useApp();
  const createMutation = useCreateChannel();
  const updateMutation = useUpdateChannel();
  const deleteMutation = useDeleteChannel();

  const handleCreate = () => {
    setModalMode('create');
    setSelectedChannel(undefined);
    form.resetFields();
    setModalOpen(true);
  };

  const handleEdit = (channel: ReleaseChannel) => {
    setModalMode('edit');
    setSelectedChannel(channel);
    form.setFieldsValue({
      name: channel.name,
      description: channel.description || '',
    });
    setModalOpen(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await deleteMutation.mutateAsync(id);
      message.success('Channel deleted successfully');
    } catch (error: any) {
      const errorMessage = error.response?.data?.error || error.message || 'Failed to delete channel';
      message.error(errorMessage);
    }
  };

  const handleModalSubmit = async () => {
    try {
      const values = await form.validateFields();

      if (modalMode === 'create') {
        await createMutation.mutateAsync(values);
        message.success('Channel created successfully');
      } else if (selectedChannel) {
        await updateMutation.mutateAsync({
          id: selectedChannel.id,
          data: values,
        });
        message.success('Channel updated successfully');
      }

      setModalOpen(false);
      setSelectedChannel(undefined);
      form.resetFields();
    } catch (error: any) {
      if (error.errorFields) {
        // Form validation errors
        return;
      }
      const errorMessage = error.response?.data?.error || error.message || 'Operation failed';
      message.error(errorMessage);
    }
  };

  const handleModalCancel = () => {
    setModalOpen(false);
    setSelectedChannel(undefined);
    form.resetFields();
  };

  const columns: ColumnsType<ReleaseChannel> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      sorter: (a, b) => a.id - b.id,
      render: (id: number) => (
        <span style={{ color: '#94a3b8', fontWeight: 500, fontSize: 13 }}>#{id}</span>
      ),
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
      render: (name: string) => (
        <Space>
          <TagsOutlined style={{ color: '#60a5fa' }} />
          <span style={{ fontWeight: 600, fontSize: 14 }}>{name}</span>
        </Space>
      ),
    },
    {
      title: 'Description',
      dataIndex: 'description',
      key: 'description',
      render: (description: string | null) => (
        <span style={{ color: '#94a3b8' }}>
          {description || <em style={{ color: '#64748b' }}>No description</em>}
        </span>
      ),
    },
    {
      title: 'Created',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 180,
      render: (created: string) => (
        <Tooltip title={dayjs(created).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(created).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.created_at).unix() - dayjs(b.created_at).unix(),
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 140,
      fixed: 'right',
      align: 'center',
      render: (_, record) => (
        <Space size={8}>
          <Tooltip title="Edit Channel">
            <Button
              type="default"
              icon={<EditOutlined />}
              onClick={() => handleEdit(record)}
              size="middle"
              style={{
                borderRadius: 6,
                borderColor: '#475569',
              }}
            />
          </Tooltip>
          <Popconfirm
            title="Delete channel?"
            description={
              <div>
                <p>This will permanently remove this channel.</p>
                <p style={{ color: '#ef4444', marginTop: 8 }}>
                  Warning: Arcades and versions using this channel will be affected.
                </p>
              </div>
            }
            onConfirm={() => handleDelete(record.id)}
            okText="Delete"
            okButtonProps={{ danger: true }}
            cancelText="Cancel"
          >
            <Tooltip title="Delete Channel">
              <Button
                danger
                icon={<DeleteOutlined />}
                size="middle"
                style={{
                  borderRadius: 6,
                }}
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header Section */}
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, fontSize: 28, fontWeight: 600 }}>
            Release Channels
          </Title>
          <div style={{ marginTop: 8, color: '#94a3b8', fontSize: 14 }}>
            {channels.length} channel{channels.length !== 1 ? 's' : ''} total
          </div>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
          style={{ minHeight: 42 }}
        >
          Create Channel
        </Button>
      </Flex>

      {/* Main Content Card */}
      <Card
        style={{
          borderRadius: 12,
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.2), 0 2px 4px -2px rgb(0 0 0 / 0.2)',
        }}
      >
        {/* Data Table */}
        <Table
          columns={columns}
          dataSource={channels}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `${total} channel${total !== 1 ? 's' : ''} total`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50'],
            style: { marginTop: 16 },
          }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>

      {/* Create/Edit Modal */}
      <Modal
        title={modalMode === 'create' ? 'Create Release Channel' : 'Edit Release Channel'}
        open={modalOpen}
        onOk={handleModalSubmit}
        onCancel={handleModalCancel}
        confirmLoading={createMutation.isPending || updateMutation.isPending}
        okText={modalMode === 'create' ? 'Create' : 'Update'}
        width={600}
      >
        <Form
          form={form}
          layout="vertical"
          style={{ marginTop: 24 }}
        >
          <Form.Item
            label="Channel Name"
            name="name"
            rules={[
              { required: true, message: 'Please enter a channel name' },
              { min: 2, message: 'Name must be at least 2 characters' },
              { max: 50, message: 'Name must be at most 50 characters' },
              {
                pattern: /^[a-z0-9-]+$/,
                message: 'Name must be lowercase letters, numbers, and hyphens only',
              },
            ]}
          >
            <Input
              placeholder="e.g., production, staging, beta"
              size="large"
              disabled={modalMode === 'edit'}
            />
          </Form.Item>

          {modalMode === 'edit' && (
            <div
              style={{
                padding: 12,
                backgroundColor: 'rgba(251, 191, 36, 0.1)',
                border: '1px solid rgba(251, 191, 36, 0.3)',
                borderRadius: 6,
                marginBottom: 16,
              }}
            >
              <p style={{ margin: 0, color: '#fbbf24', fontSize: 13 }}>
                <strong>Note:</strong> Channel names cannot be changed after creation for data integrity.
              </p>
            </div>
          )}

          <Form.Item
            label="Description"
            name="description"
            rules={[
              { max: 500, message: 'Description must be at most 500 characters' },
            ]}
          >
            <TextArea
              placeholder="Describe the purpose of this channel..."
              rows={4}
              maxLength={500}
              showCount
              size="large"
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

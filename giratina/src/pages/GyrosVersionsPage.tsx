import { useState, useMemo } from 'react';
import {
  Typography,
  Button,
  Table,
  Space,
  Input,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
  Tag,
  message,
  Progress,
  App,
} from 'antd';
import {
  PlusOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
  CheckCircleOutlined,
  StarOutlined,
  StarFilled,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import {
  useGyrosVersions,
  useSetGyrosVersionCurrent,
  useDeleteGyrosVersion,
} from '../hooks/useGyros';
import { GyrosModal } from '../components/GyrosModal';
import type { GyrosVersion } from '../types';
import { api } from '../services/api';

dayjs.extend(relativeTime);

const { Title } = Typography;

export const GyrosVersionsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [searchText, setSearchText] = useState('');
  const [isUploading, setIsUploading] = useState(false);

  const { data: versions = [], isLoading, refetch } = useGyrosVersions();
  const setCurrentMutation = useSetGyrosVersionCurrent();
  const deleteMutation = useDeleteGyrosVersion();

  const { modal: antModal } = App.useApp();

  const filteredVersions = useMemo(() => {
    return versions.filter((version) =>
      version.version.toLowerCase().includes(searchText.toLowerCase()) ||
      version.gcs_path.toLowerCase().includes(searchText.toLowerCase())
    );
  }, [versions, searchText]);

  const handleCreate = () => {
    setModalOpen(true);
  };

  const handleSetCurrent = async (id: number) => {
    await setCurrentMutation.mutateAsync(id);
  };

  const handleDelete = async (id: number) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleModalSubmit = async (values: any) => {
    try {
      const { binaryFile, version } = values;

      if (!binaryFile) {
        message.error('Binary file is required');
        return;
      }

      setIsUploading(true);
      setModalOpen(false);

      // Show progress modal
      const progressModal = antModal.info({
        title: 'Uploading Gyros Firmware',
        content: (
          <div>
            <p>Uploading {binaryFile.name}...</p>
            <Progress percent={0} status="active" />
            <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
              Please wait while the file is being uploaded.
            </p>
          </div>
        ),
        okButtonProps: { style: { display: 'none' } },
        closable: false,
        maskClosable: false,
      });

      try {
        await api.uploadGyrosBinary(version, binaryFile, (progress) => {
          progressModal.update({
            content: (
              <div>
                <p>Uploading {binaryFile.name}...</p>
                <Progress percent={progress} status="active" />
                <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
                  {progress < 100
                    ? 'Please wait while the file is being uploaded.'
                    : 'Processing...'}
                </p>
              </div>
            ),
          });
        });

        progressModal.destroy();
        await refetch();
        message.success(`Gyros version ${version} created successfully`);
      } catch (error: any) {
        progressModal.destroy();
        const errorMessage = error.response?.data?.error || error.message || 'Failed to upload Gyros firmware';
        message.error(errorMessage);
      } finally {
        setIsUploading(false);
      }
    } catch (error: any) {
      message.error(error.message || 'Failed to upload Gyros firmware');
    }
  };

  const handleModalCancel = () => {
    if (isUploading) {
      antModal.confirm({
        title: 'Upload in progress',
        content: 'Are you sure you want to cancel? The upload will be interrupted.',
        onOk: () => {
          setModalOpen(false);
          setIsUploading(false);
        },
      });
    } else {
      setModalOpen(false);
    }
  };

  const columns: ColumnsType<GyrosVersion> = [
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
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
      width: 180,
      sorter: (a, b) => a.version.localeCompare(b.version),
      render: (version: string, record: GyrosVersion) => (
        <Space size={8}>
          <Tag color="blue" style={{ fontSize: 13, padding: '4px 12px', borderRadius: 6, fontWeight: 600 }}>
            v{version}
          </Tag>
          {record.is_current && (
            <Tag
              icon={<CheckCircleOutlined />}
              color="success"
              style={{
                fontSize: 12,
                padding: '4px 12px',
                borderRadius: 6,
                fontWeight: 600,
              }}
            >
              CURRENT
            </Tag>
          )}
        </Space>
      ),
    },
    {
      title: 'GCS Path',
      dataIndex: 'gcs_path',
      key: 'gcs_path',
      render: (path: string) => (
        <Tooltip title={`Full path: ${path}/Gyros.bin`}>
          <code
            style={{
              fontSize: 12,
              padding: '4px 8px',
              backgroundColor: '#0f172a',
              borderRadius: 4,
              color: '#06b6d4',
              border: '1px solid #334155',
            }}
          >
            {path}
          </code>
        </Tooltip>
      ),
    },
    {
      title: 'Release Date',
      dataIndex: 'release_date',
      key: 'release_date',
      width: 180,
      render: (date: string) => (
        <Tooltip title={dayjs(date).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(date).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.release_date).unix() - dayjs(b.release_date).unix(),
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
      width: 170,
      fixed: 'right',
      align: 'center',
      render: (_, record) => (
        <Space size={8}>
          {!record.is_current && (
            <Tooltip title="Set as Current Version">
              <Button
                type="default"
                icon={<StarOutlined />}
                onClick={() => handleSetCurrent(record.id)}
                loading={setCurrentMutation.isPending}
                size="middle"
                style={{
                  borderRadius: 6,
                  borderColor: '#f59e0b',
                  color: '#f59e0b',
                }}
              />
            </Tooltip>
          )}
          {record.is_current && (
            <Tooltip title="Currently Active">
              <Button
                type="default"
                icon={<StarFilled />}
                size="middle"
                disabled
                style={{
                  borderRadius: 6,
                  borderColor: '#f59e0b',
                  color: '#f59e0b',
                }}
              />
            </Tooltip>
          )}
          <Popconfirm
            title="Delete version?"
            description={
              record.is_current
                ? "Cannot delete the current version. Set another version as current first."
                : "Are you sure you want to delete this Gyros version?"
            }
            onConfirm={() => handleDelete(record.id)}
            okText="Delete"
            okButtonProps={{ danger: true, disabled: record.is_current }}
            cancelText="Cancel"
            disabled={record.is_current}
          >
            <Tooltip title={record.is_current ? "Cannot delete current version" : "Delete Version"}>
              <Button
                danger
                icon={<DeleteOutlined />}
                size="middle"
                disabled={record.is_current}
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

  const currentVersion = versions.find(v => v.is_current);

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header Section */}
      <Flex justify="space-between" align="flex-start" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, marginBottom: 12, fontSize: 28, fontWeight: 600 }}>
            Gyros Versions
          </Title>
          <Flex gap={12} wrap="wrap" align="center">
            <div style={{ color: '#94a3b8', fontSize: 14 }}>
              {filteredVersions.length} version{filteredVersions.length !== 1 ? 's' : ''} total
            </div>
            {currentVersion && (
              <Tag
                icon={<CheckCircleOutlined />}
                color="success"
                style={{
                  fontSize: 13,
                  padding: '4px 12px',
                  borderRadius: 6,
                  fontWeight: 500,
                }}
              >
                Current: v{currentVersion.version}
              </Tag>
            )}
          </Flex>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
          style={{ minHeight: 42 }}
        >
          Upload Version
        </Button>
      </Flex>

      {/* Main Content Card */}
      <Card
        style={{
          borderRadius: 12,
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.2), 0 2px 4px -2px rgb(0 0 0 / 0.2)',
        }}
      >
        {/* Search Bar */}
        <Flex gap={12} style={{ marginBottom: 20 }} wrap="wrap" align="center">
          <Input
            placeholder="Search versions, paths..."
            prefix={<SearchOutlined style={{ color: '#64748b' }} />}
            allowClear
            style={{ maxWidth: 400, flex: 1 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            size="large"
          />
          <Tooltip title="Refresh Data">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
              size="large"
            >
              Refresh
            </Button>
          </Tooltip>
        </Flex>

        {/* Data Table */}
        <Table
          columns={columns}
          dataSource={filteredVersions}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `${total} version${total !== 1 ? 's' : ''} total`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50'],
            style: { marginTop: 16 },
          }}
          scroll={{ x: 1000 }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>

      <GyrosModal
        open={modalOpen}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={isUploading}
      />

    </div>
  );
};

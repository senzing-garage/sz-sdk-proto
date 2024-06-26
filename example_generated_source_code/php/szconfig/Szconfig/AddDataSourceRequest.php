<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: szconfig.proto

namespace Szconfig;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>szconfig.AddDataSourceRequest</code>
 */
class AddDataSourceRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>int64 configHandle = 1;</code>
     */
    protected $configHandle = 0;
    /**
     * Generated from protobuf field <code>string dataSourceCode = 2;</code>
     */
    protected $dataSourceCode = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $configHandle
     *     @type string $dataSourceCode
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Szconfig::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>int64 configHandle = 1;</code>
     * @return int|string
     */
    public function getConfigHandle()
    {
        return $this->configHandle;
    }

    /**
     * Generated from protobuf field <code>int64 configHandle = 1;</code>
     * @param int|string $var
     * @return $this
     */
    public function setConfigHandle($var)
    {
        GPBUtil::checkInt64($var);
        $this->configHandle = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string dataSourceCode = 2;</code>
     * @return string
     */
    public function getDataSourceCode()
    {
        return $this->dataSourceCode;
    }

    /**
     * Generated from protobuf field <code>string dataSourceCode = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setDataSourceCode($var)
    {
        GPBUtil::checkString($var, True);
        $this->dataSourceCode = $var;

        return $this;
    }

}


/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
"use strict";

var $protobuf = require("protobufjs/minimal");

// Common aliases
var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

$root.CouncilCandidacyNoteMetadata = (function() {

    /**
     * Properties of a CouncilCandidacyNoteMetadata.
     * @exports ICouncilCandidacyNoteMetadata
     * @interface ICouncilCandidacyNoteMetadata
     * @property {string|null} [header] CouncilCandidacyNoteMetadata header
     * @property {Array.<string>|null} [bulletPoints] CouncilCandidacyNoteMetadata bulletPoints
     * @property {string|null} [bannerImageUri] CouncilCandidacyNoteMetadata bannerImageUri
     * @property {string|null} [description] CouncilCandidacyNoteMetadata description
     */

    /**
     * Constructs a new CouncilCandidacyNoteMetadata.
     * @exports CouncilCandidacyNoteMetadata
     * @classdesc Represents a CouncilCandidacyNoteMetadata.
     * @implements ICouncilCandidacyNoteMetadata
     * @constructor
     * @param {ICouncilCandidacyNoteMetadata=} [properties] Properties to set
     */
    function CouncilCandidacyNoteMetadata(properties) {
        this.bulletPoints = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * CouncilCandidacyNoteMetadata header.
     * @member {string} header
     * @memberof CouncilCandidacyNoteMetadata
     * @instance
     */
    CouncilCandidacyNoteMetadata.prototype.header = "";

    /**
     * CouncilCandidacyNoteMetadata bulletPoints.
     * @member {Array.<string>} bulletPoints
     * @memberof CouncilCandidacyNoteMetadata
     * @instance
     */
    CouncilCandidacyNoteMetadata.prototype.bulletPoints = $util.emptyArray;

    /**
     * CouncilCandidacyNoteMetadata bannerImageUri.
     * @member {string} bannerImageUri
     * @memberof CouncilCandidacyNoteMetadata
     * @instance
     */
    CouncilCandidacyNoteMetadata.prototype.bannerImageUri = "";

    /**
     * CouncilCandidacyNoteMetadata description.
     * @member {string} description
     * @memberof CouncilCandidacyNoteMetadata
     * @instance
     */
    CouncilCandidacyNoteMetadata.prototype.description = "";

    /**
     * Creates a new CouncilCandidacyNoteMetadata instance using the specified properties.
     * @function create
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {ICouncilCandidacyNoteMetadata=} [properties] Properties to set
     * @returns {CouncilCandidacyNoteMetadata} CouncilCandidacyNoteMetadata instance
     */
    CouncilCandidacyNoteMetadata.create = function create(properties) {
        return new CouncilCandidacyNoteMetadata(properties);
    };

    /**
     * Encodes the specified CouncilCandidacyNoteMetadata message. Does not implicitly {@link CouncilCandidacyNoteMetadata.verify|verify} messages.
     * @function encode
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {ICouncilCandidacyNoteMetadata} message CouncilCandidacyNoteMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    CouncilCandidacyNoteMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.header != null && Object.hasOwnProperty.call(message, "header"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.header);
        if (message.bulletPoints != null && message.bulletPoints.length)
            for (var i = 0; i < message.bulletPoints.length; ++i)
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.bulletPoints[i]);
        if (message.bannerImageUri != null && Object.hasOwnProperty.call(message, "bannerImageUri"))
            writer.uint32(/* id 3, wireType 2 =*/26).string(message.bannerImageUri);
        if (message.description != null && Object.hasOwnProperty.call(message, "description"))
            writer.uint32(/* id 4, wireType 2 =*/34).string(message.description);
        return writer;
    };

    /**
     * Encodes the specified CouncilCandidacyNoteMetadata message, length delimited. Does not implicitly {@link CouncilCandidacyNoteMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {ICouncilCandidacyNoteMetadata} message CouncilCandidacyNoteMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    CouncilCandidacyNoteMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a CouncilCandidacyNoteMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {CouncilCandidacyNoteMetadata} CouncilCandidacyNoteMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    CouncilCandidacyNoteMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.CouncilCandidacyNoteMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.header = reader.string();
                break;
            case 2:
                if (!(message.bulletPoints && message.bulletPoints.length))
                    message.bulletPoints = [];
                message.bulletPoints.push(reader.string());
                break;
            case 3:
                message.bannerImageUri = reader.string();
                break;
            case 4:
                message.description = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a CouncilCandidacyNoteMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {CouncilCandidacyNoteMetadata} CouncilCandidacyNoteMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    CouncilCandidacyNoteMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a CouncilCandidacyNoteMetadata message.
     * @function verify
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    CouncilCandidacyNoteMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.header != null && message.hasOwnProperty("header"))
            if (!$util.isString(message.header))
                return "header: string expected";
        if (message.bulletPoints != null && message.hasOwnProperty("bulletPoints")) {
            if (!Array.isArray(message.bulletPoints))
                return "bulletPoints: array expected";
            for (var i = 0; i < message.bulletPoints.length; ++i)
                if (!$util.isString(message.bulletPoints[i]))
                    return "bulletPoints: string[] expected";
        }
        if (message.bannerImageUri != null && message.hasOwnProperty("bannerImageUri"))
            if (!$util.isString(message.bannerImageUri))
                return "bannerImageUri: string expected";
        if (message.description != null && message.hasOwnProperty("description"))
            if (!$util.isString(message.description))
                return "description: string expected";
        return null;
    };

    /**
     * Creates a CouncilCandidacyNoteMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {CouncilCandidacyNoteMetadata} CouncilCandidacyNoteMetadata
     */
    CouncilCandidacyNoteMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.CouncilCandidacyNoteMetadata)
            return object;
        var message = new $root.CouncilCandidacyNoteMetadata();
        if (object.header != null)
            message.header = String(object.header);
        if (object.bulletPoints) {
            if (!Array.isArray(object.bulletPoints))
                throw TypeError(".CouncilCandidacyNoteMetadata.bulletPoints: array expected");
            message.bulletPoints = [];
            for (var i = 0; i < object.bulletPoints.length; ++i)
                message.bulletPoints[i] = String(object.bulletPoints[i]);
        }
        if (object.bannerImageUri != null)
            message.bannerImageUri = String(object.bannerImageUri);
        if (object.description != null)
            message.description = String(object.description);
        return message;
    };

    /**
     * Creates a plain object from a CouncilCandidacyNoteMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof CouncilCandidacyNoteMetadata
     * @static
     * @param {CouncilCandidacyNoteMetadata} message CouncilCandidacyNoteMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    CouncilCandidacyNoteMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.bulletPoints = [];
        if (options.defaults) {
            object.header = "";
            object.bannerImageUri = "";
            object.description = "";
        }
        if (message.header != null && message.hasOwnProperty("header"))
            object.header = message.header;
        if (message.bulletPoints && message.bulletPoints.length) {
            object.bulletPoints = [];
            for (var j = 0; j < message.bulletPoints.length; ++j)
                object.bulletPoints[j] = message.bulletPoints[j];
        }
        if (message.bannerImageUri != null && message.hasOwnProperty("bannerImageUri"))
            object.bannerImageUri = message.bannerImageUri;
        if (message.description != null && message.hasOwnProperty("description"))
            object.description = message.description;
        return object;
    };

    /**
     * Converts this CouncilCandidacyNoteMetadata to JSON.
     * @function toJSON
     * @memberof CouncilCandidacyNoteMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    CouncilCandidacyNoteMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return CouncilCandidacyNoteMetadata;
})();

$root.ForumPostReaction = (function() {

    /**
     * Properties of a ForumPostReaction.
     * @exports IForumPostReaction
     * @interface IForumPostReaction
     */

    /**
     * Constructs a new ForumPostReaction.
     * @exports ForumPostReaction
     * @classdesc Represents a ForumPostReaction.
     * @implements IForumPostReaction
     * @constructor
     * @param {IForumPostReaction=} [properties] Properties to set
     */
    function ForumPostReaction(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * Creates a new ForumPostReaction instance using the specified properties.
     * @function create
     * @memberof ForumPostReaction
     * @static
     * @param {IForumPostReaction=} [properties] Properties to set
     * @returns {ForumPostReaction} ForumPostReaction instance
     */
    ForumPostReaction.create = function create(properties) {
        return new ForumPostReaction(properties);
    };

    /**
     * Encodes the specified ForumPostReaction message. Does not implicitly {@link ForumPostReaction.verify|verify} messages.
     * @function encode
     * @memberof ForumPostReaction
     * @static
     * @param {IForumPostReaction} message ForumPostReaction message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ForumPostReaction.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        return writer;
    };

    /**
     * Encodes the specified ForumPostReaction message, length delimited. Does not implicitly {@link ForumPostReaction.verify|verify} messages.
     * @function encodeDelimited
     * @memberof ForumPostReaction
     * @static
     * @param {IForumPostReaction} message ForumPostReaction message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ForumPostReaction.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a ForumPostReaction message from the specified reader or buffer.
     * @function decode
     * @memberof ForumPostReaction
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {ForumPostReaction} ForumPostReaction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ForumPostReaction.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.ForumPostReaction();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a ForumPostReaction message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof ForumPostReaction
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {ForumPostReaction} ForumPostReaction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ForumPostReaction.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a ForumPostReaction message.
     * @function verify
     * @memberof ForumPostReaction
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    ForumPostReaction.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        return null;
    };

    /**
     * Creates a ForumPostReaction message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof ForumPostReaction
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {ForumPostReaction} ForumPostReaction
     */
    ForumPostReaction.fromObject = function fromObject(object) {
        if (object instanceof $root.ForumPostReaction)
            return object;
        return new $root.ForumPostReaction();
    };

    /**
     * Creates a plain object from a ForumPostReaction message. Also converts values to other types if specified.
     * @function toObject
     * @memberof ForumPostReaction
     * @static
     * @param {ForumPostReaction} message ForumPostReaction
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    ForumPostReaction.toObject = function toObject() {
        return {};
    };

    /**
     * Converts this ForumPostReaction to JSON.
     * @function toJSON
     * @memberof ForumPostReaction
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    ForumPostReaction.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    /**
     * Reaction enum.
     * @name ForumPostReaction.Reaction
     * @enum {number}
     * @property {number} CANCEL=0 CANCEL value
     * @property {number} LIKE=1 LIKE value
     */
    ForumPostReaction.Reaction = (function() {
        var valuesById = {}, values = Object.create(valuesById);
        values[valuesById[0] = "CANCEL"] = 0;
        values[valuesById[1] = "LIKE"] = 1;
        return values;
    })();

    return ForumPostReaction;
})();

$root.ForumPostMetadata = (function() {

    /**
     * Properties of a ForumPostMetadata.
     * @exports IForumPostMetadata
     * @interface IForumPostMetadata
     * @property {string|null} [text] ForumPostMetadata text
     * @property {number|null} [repliesTo] ForumPostMetadata repliesTo
     */

    /**
     * Constructs a new ForumPostMetadata.
     * @exports ForumPostMetadata
     * @classdesc Represents a ForumPostMetadata.
     * @implements IForumPostMetadata
     * @constructor
     * @param {IForumPostMetadata=} [properties] Properties to set
     */
    function ForumPostMetadata(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * ForumPostMetadata text.
     * @member {string} text
     * @memberof ForumPostMetadata
     * @instance
     */
    ForumPostMetadata.prototype.text = "";

    /**
     * ForumPostMetadata repliesTo.
     * @member {number} repliesTo
     * @memberof ForumPostMetadata
     * @instance
     */
    ForumPostMetadata.prototype.repliesTo = 0;

    /**
     * Creates a new ForumPostMetadata instance using the specified properties.
     * @function create
     * @memberof ForumPostMetadata
     * @static
     * @param {IForumPostMetadata=} [properties] Properties to set
     * @returns {ForumPostMetadata} ForumPostMetadata instance
     */
    ForumPostMetadata.create = function create(properties) {
        return new ForumPostMetadata(properties);
    };

    /**
     * Encodes the specified ForumPostMetadata message. Does not implicitly {@link ForumPostMetadata.verify|verify} messages.
     * @function encode
     * @memberof ForumPostMetadata
     * @static
     * @param {IForumPostMetadata} message ForumPostMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ForumPostMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.text != null && Object.hasOwnProperty.call(message, "text"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.text);
        if (message.repliesTo != null && Object.hasOwnProperty.call(message, "repliesTo"))
            writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.repliesTo);
        return writer;
    };

    /**
     * Encodes the specified ForumPostMetadata message, length delimited. Does not implicitly {@link ForumPostMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof ForumPostMetadata
     * @static
     * @param {IForumPostMetadata} message ForumPostMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ForumPostMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a ForumPostMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof ForumPostMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {ForumPostMetadata} ForumPostMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ForumPostMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.ForumPostMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.text = reader.string();
                break;
            case 2:
                message.repliesTo = reader.uint32();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a ForumPostMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof ForumPostMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {ForumPostMetadata} ForumPostMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ForumPostMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a ForumPostMetadata message.
     * @function verify
     * @memberof ForumPostMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    ForumPostMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.text != null && message.hasOwnProperty("text"))
            if (!$util.isString(message.text))
                return "text: string expected";
        if (message.repliesTo != null && message.hasOwnProperty("repliesTo"))
            if (!$util.isInteger(message.repliesTo))
                return "repliesTo: integer expected";
        return null;
    };

    /**
     * Creates a ForumPostMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof ForumPostMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {ForumPostMetadata} ForumPostMetadata
     */
    ForumPostMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.ForumPostMetadata)
            return object;
        var message = new $root.ForumPostMetadata();
        if (object.text != null)
            message.text = String(object.text);
        if (object.repliesTo != null)
            message.repliesTo = object.repliesTo >>> 0;
        return message;
    };

    /**
     * Creates a plain object from a ForumPostMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof ForumPostMetadata
     * @static
     * @param {ForumPostMetadata} message ForumPostMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    ForumPostMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.text = "";
            object.repliesTo = 0;
        }
        if (message.text != null && message.hasOwnProperty("text"))
            object.text = message.text;
        if (message.repliesTo != null && message.hasOwnProperty("repliesTo"))
            object.repliesTo = message.repliesTo;
        return object;
    };

    /**
     * Converts this ForumPostMetadata to JSON.
     * @function toJSON
     * @memberof ForumPostMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    ForumPostMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return ForumPostMetadata;
})();

$root.MembershipMetadata = (function() {

    /**
     * Properties of a MembershipMetadata.
     * @exports IMembershipMetadata
     * @interface IMembershipMetadata
     * @property {string|null} [name] MembershipMetadata name
     * @property {number|null} [avatar] MembershipMetadata avatar
     * @property {string|null} [about] MembershipMetadata about
     */

    /**
     * Constructs a new MembershipMetadata.
     * @exports MembershipMetadata
     * @classdesc Represents a MembershipMetadata.
     * @implements IMembershipMetadata
     * @constructor
     * @param {IMembershipMetadata=} [properties] Properties to set
     */
    function MembershipMetadata(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * MembershipMetadata name.
     * @member {string} name
     * @memberof MembershipMetadata
     * @instance
     */
    MembershipMetadata.prototype.name = "";

    /**
     * MembershipMetadata avatar.
     * @member {number} avatar
     * @memberof MembershipMetadata
     * @instance
     */
    MembershipMetadata.prototype.avatar = 0;

    /**
     * MembershipMetadata about.
     * @member {string} about
     * @memberof MembershipMetadata
     * @instance
     */
    MembershipMetadata.prototype.about = "";

    /**
     * Creates a new MembershipMetadata instance using the specified properties.
     * @function create
     * @memberof MembershipMetadata
     * @static
     * @param {IMembershipMetadata=} [properties] Properties to set
     * @returns {MembershipMetadata} MembershipMetadata instance
     */
    MembershipMetadata.create = function create(properties) {
        return new MembershipMetadata(properties);
    };

    /**
     * Encodes the specified MembershipMetadata message. Does not implicitly {@link MembershipMetadata.verify|verify} messages.
     * @function encode
     * @memberof MembershipMetadata
     * @static
     * @param {IMembershipMetadata} message MembershipMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    MembershipMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.name != null && Object.hasOwnProperty.call(message, "name"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
        if (message.avatar != null && Object.hasOwnProperty.call(message, "avatar"))
            writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.avatar);
        if (message.about != null && Object.hasOwnProperty.call(message, "about"))
            writer.uint32(/* id 3, wireType 2 =*/26).string(message.about);
        return writer;
    };

    /**
     * Encodes the specified MembershipMetadata message, length delimited. Does not implicitly {@link MembershipMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof MembershipMetadata
     * @static
     * @param {IMembershipMetadata} message MembershipMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    MembershipMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a MembershipMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof MembershipMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {MembershipMetadata} MembershipMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    MembershipMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.MembershipMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.name = reader.string();
                break;
            case 2:
                message.avatar = reader.uint32();
                break;
            case 3:
                message.about = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a MembershipMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof MembershipMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {MembershipMetadata} MembershipMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    MembershipMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a MembershipMetadata message.
     * @function verify
     * @memberof MembershipMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    MembershipMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.name != null && message.hasOwnProperty("name"))
            if (!$util.isString(message.name))
                return "name: string expected";
        if (message.avatar != null && message.hasOwnProperty("avatar"))
            if (!$util.isInteger(message.avatar))
                return "avatar: integer expected";
        if (message.about != null && message.hasOwnProperty("about"))
            if (!$util.isString(message.about))
                return "about: string expected";
        return null;
    };

    /**
     * Creates a MembershipMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof MembershipMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {MembershipMetadata} MembershipMetadata
     */
    MembershipMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.MembershipMetadata)
            return object;
        var message = new $root.MembershipMetadata();
        if (object.name != null)
            message.name = String(object.name);
        if (object.avatar != null)
            message.avatar = object.avatar >>> 0;
        if (object.about != null)
            message.about = String(object.about);
        return message;
    };

    /**
     * Creates a plain object from a MembershipMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof MembershipMetadata
     * @static
     * @param {MembershipMetadata} message MembershipMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    MembershipMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.name = "";
            object.avatar = 0;
            object.about = "";
        }
        if (message.name != null && message.hasOwnProperty("name"))
            object.name = message.name;
        if (message.avatar != null && message.hasOwnProperty("avatar"))
            object.avatar = message.avatar;
        if (message.about != null && message.hasOwnProperty("about"))
            object.about = message.about;
        return object;
    };

    /**
     * Converts this MembershipMetadata to JSON.
     * @function toJSON
     * @memberof MembershipMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    MembershipMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return MembershipMetadata;
})();

$root.OpeningMetadata = (function() {

    /**
     * Properties of an OpeningMetadata.
     * @exports IOpeningMetadata
     * @interface IOpeningMetadata
     * @property {string|null} [shortDescription] OpeningMetadata shortDescription
     * @property {string|null} [description] OpeningMetadata description
     * @property {number|null} [hiringLimit] OpeningMetadata hiringLimit
     * @property {number|null} [expectedEndingTimestamp] OpeningMetadata expectedEndingTimestamp
     * @property {string|null} [applicationDetails] OpeningMetadata applicationDetails
     * @property {Array.<OpeningMetadata.IApplicationFormQuestion>|null} [applicationFormQuestions] OpeningMetadata applicationFormQuestions
     */

    /**
     * Constructs a new OpeningMetadata.
     * @exports OpeningMetadata
     * @classdesc Represents an OpeningMetadata.
     * @implements IOpeningMetadata
     * @constructor
     * @param {IOpeningMetadata=} [properties] Properties to set
     */
    function OpeningMetadata(properties) {
        this.applicationFormQuestions = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * OpeningMetadata shortDescription.
     * @member {string} shortDescription
     * @memberof OpeningMetadata
     * @instance
     */
    OpeningMetadata.prototype.shortDescription = "";

    /**
     * OpeningMetadata description.
     * @member {string} description
     * @memberof OpeningMetadata
     * @instance
     */
    OpeningMetadata.prototype.description = "";

    /**
     * OpeningMetadata hiringLimit.
     * @member {number} hiringLimit
     * @memberof OpeningMetadata
     * @instance
     */
    OpeningMetadata.prototype.hiringLimit = 0;

    /**
     * OpeningMetadata expectedEndingTimestamp.
     * @member {number} expectedEndingTimestamp
     * @memberof OpeningMetadata
     * @instance
     */
    OpeningMetadata.prototype.expectedEndingTimestamp = 0;

    /**
     * OpeningMetadata applicationDetails.
     * @member {string} applicationDetails
     * @memberof OpeningMetadata
     * @instance
     */
    OpeningMetadata.prototype.applicationDetails = "";

    /**
     * OpeningMetadata applicationFormQuestions.
     * @member {Array.<OpeningMetadata.IApplicationFormQuestion>} applicationFormQuestions
     * @memberof OpeningMetadata
     * @instance
     */
    OpeningMetadata.prototype.applicationFormQuestions = $util.emptyArray;

    /**
     * Creates a new OpeningMetadata instance using the specified properties.
     * @function create
     * @memberof OpeningMetadata
     * @static
     * @param {IOpeningMetadata=} [properties] Properties to set
     * @returns {OpeningMetadata} OpeningMetadata instance
     */
    OpeningMetadata.create = function create(properties) {
        return new OpeningMetadata(properties);
    };

    /**
     * Encodes the specified OpeningMetadata message. Does not implicitly {@link OpeningMetadata.verify|verify} messages.
     * @function encode
     * @memberof OpeningMetadata
     * @static
     * @param {IOpeningMetadata} message OpeningMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    OpeningMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.shortDescription != null && Object.hasOwnProperty.call(message, "shortDescription"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.shortDescription);
        if (message.description != null && Object.hasOwnProperty.call(message, "description"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.description);
        if (message.hiringLimit != null && Object.hasOwnProperty.call(message, "hiringLimit"))
            writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.hiringLimit);
        if (message.expectedEndingTimestamp != null && Object.hasOwnProperty.call(message, "expectedEndingTimestamp"))
            writer.uint32(/* id 4, wireType 0 =*/32).uint32(message.expectedEndingTimestamp);
        if (message.applicationDetails != null && Object.hasOwnProperty.call(message, "applicationDetails"))
            writer.uint32(/* id 5, wireType 2 =*/42).string(message.applicationDetails);
        if (message.applicationFormQuestions != null && message.applicationFormQuestions.length)
            for (var i = 0; i < message.applicationFormQuestions.length; ++i)
                $root.OpeningMetadata.ApplicationFormQuestion.encode(message.applicationFormQuestions[i], writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified OpeningMetadata message, length delimited. Does not implicitly {@link OpeningMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof OpeningMetadata
     * @static
     * @param {IOpeningMetadata} message OpeningMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    OpeningMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes an OpeningMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof OpeningMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {OpeningMetadata} OpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    OpeningMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.OpeningMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.shortDescription = reader.string();
                break;
            case 2:
                message.description = reader.string();
                break;
            case 3:
                message.hiringLimit = reader.uint32();
                break;
            case 4:
                message.expectedEndingTimestamp = reader.uint32();
                break;
            case 5:
                message.applicationDetails = reader.string();
                break;
            case 6:
                if (!(message.applicationFormQuestions && message.applicationFormQuestions.length))
                    message.applicationFormQuestions = [];
                message.applicationFormQuestions.push($root.OpeningMetadata.ApplicationFormQuestion.decode(reader, reader.uint32()));
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes an OpeningMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof OpeningMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {OpeningMetadata} OpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    OpeningMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies an OpeningMetadata message.
     * @function verify
     * @memberof OpeningMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    OpeningMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.shortDescription != null && message.hasOwnProperty("shortDescription"))
            if (!$util.isString(message.shortDescription))
                return "shortDescription: string expected";
        if (message.description != null && message.hasOwnProperty("description"))
            if (!$util.isString(message.description))
                return "description: string expected";
        if (message.hiringLimit != null && message.hasOwnProperty("hiringLimit"))
            if (!$util.isInteger(message.hiringLimit))
                return "hiringLimit: integer expected";
        if (message.expectedEndingTimestamp != null && message.hasOwnProperty("expectedEndingTimestamp"))
            if (!$util.isInteger(message.expectedEndingTimestamp))
                return "expectedEndingTimestamp: integer expected";
        if (message.applicationDetails != null && message.hasOwnProperty("applicationDetails"))
            if (!$util.isString(message.applicationDetails))
                return "applicationDetails: string expected";
        if (message.applicationFormQuestions != null && message.hasOwnProperty("applicationFormQuestions")) {
            if (!Array.isArray(message.applicationFormQuestions))
                return "applicationFormQuestions: array expected";
            for (var i = 0; i < message.applicationFormQuestions.length; ++i) {
                var error = $root.OpeningMetadata.ApplicationFormQuestion.verify(message.applicationFormQuestions[i]);
                if (error)
                    return "applicationFormQuestions." + error;
            }
        }
        return null;
    };

    /**
     * Creates an OpeningMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof OpeningMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {OpeningMetadata} OpeningMetadata
     */
    OpeningMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.OpeningMetadata)
            return object;
        var message = new $root.OpeningMetadata();
        if (object.shortDescription != null)
            message.shortDescription = String(object.shortDescription);
        if (object.description != null)
            message.description = String(object.description);
        if (object.hiringLimit != null)
            message.hiringLimit = object.hiringLimit >>> 0;
        if (object.expectedEndingTimestamp != null)
            message.expectedEndingTimestamp = object.expectedEndingTimestamp >>> 0;
        if (object.applicationDetails != null)
            message.applicationDetails = String(object.applicationDetails);
        if (object.applicationFormQuestions) {
            if (!Array.isArray(object.applicationFormQuestions))
                throw TypeError(".OpeningMetadata.applicationFormQuestions: array expected");
            message.applicationFormQuestions = [];
            for (var i = 0; i < object.applicationFormQuestions.length; ++i) {
                if (typeof object.applicationFormQuestions[i] !== "object")
                    throw TypeError(".OpeningMetadata.applicationFormQuestions: object expected");
                message.applicationFormQuestions[i] = $root.OpeningMetadata.ApplicationFormQuestion.fromObject(object.applicationFormQuestions[i]);
            }
        }
        return message;
    };

    /**
     * Creates a plain object from an OpeningMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof OpeningMetadata
     * @static
     * @param {OpeningMetadata} message OpeningMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    OpeningMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.applicationFormQuestions = [];
        if (options.defaults) {
            object.shortDescription = "";
            object.description = "";
            object.hiringLimit = 0;
            object.expectedEndingTimestamp = 0;
            object.applicationDetails = "";
        }
        if (message.shortDescription != null && message.hasOwnProperty("shortDescription"))
            object.shortDescription = message.shortDescription;
        if (message.description != null && message.hasOwnProperty("description"))
            object.description = message.description;
        if (message.hiringLimit != null && message.hasOwnProperty("hiringLimit"))
            object.hiringLimit = message.hiringLimit;
        if (message.expectedEndingTimestamp != null && message.hasOwnProperty("expectedEndingTimestamp"))
            object.expectedEndingTimestamp = message.expectedEndingTimestamp;
        if (message.applicationDetails != null && message.hasOwnProperty("applicationDetails"))
            object.applicationDetails = message.applicationDetails;
        if (message.applicationFormQuestions && message.applicationFormQuestions.length) {
            object.applicationFormQuestions = [];
            for (var j = 0; j < message.applicationFormQuestions.length; ++j)
                object.applicationFormQuestions[j] = $root.OpeningMetadata.ApplicationFormQuestion.toObject(message.applicationFormQuestions[j], options);
        }
        return object;
    };

    /**
     * Converts this OpeningMetadata to JSON.
     * @function toJSON
     * @memberof OpeningMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    OpeningMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    OpeningMetadata.ApplicationFormQuestion = (function() {

        /**
         * Properties of an ApplicationFormQuestion.
         * @memberof OpeningMetadata
         * @interface IApplicationFormQuestion
         * @property {string|null} [question] ApplicationFormQuestion question
         * @property {OpeningMetadata.ApplicationFormQuestion.InputType|null} [type] ApplicationFormQuestion type
         */

        /**
         * Constructs a new ApplicationFormQuestion.
         * @memberof OpeningMetadata
         * @classdesc Represents an ApplicationFormQuestion.
         * @implements IApplicationFormQuestion
         * @constructor
         * @param {OpeningMetadata.IApplicationFormQuestion=} [properties] Properties to set
         */
        function ApplicationFormQuestion(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * ApplicationFormQuestion question.
         * @member {string} question
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @instance
         */
        ApplicationFormQuestion.prototype.question = "";

        /**
         * ApplicationFormQuestion type.
         * @member {OpeningMetadata.ApplicationFormQuestion.InputType} type
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @instance
         */
        ApplicationFormQuestion.prototype.type = 0;

        /**
         * Creates a new ApplicationFormQuestion instance using the specified properties.
         * @function create
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {OpeningMetadata.IApplicationFormQuestion=} [properties] Properties to set
         * @returns {OpeningMetadata.ApplicationFormQuestion} ApplicationFormQuestion instance
         */
        ApplicationFormQuestion.create = function create(properties) {
            return new ApplicationFormQuestion(properties);
        };

        /**
         * Encodes the specified ApplicationFormQuestion message. Does not implicitly {@link OpeningMetadata.ApplicationFormQuestion.verify|verify} messages.
         * @function encode
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {OpeningMetadata.IApplicationFormQuestion} message ApplicationFormQuestion message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ApplicationFormQuestion.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.question != null && Object.hasOwnProperty.call(message, "question"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.question);
            if (message.type != null && Object.hasOwnProperty.call(message, "type"))
                writer.uint32(/* id 2, wireType 0 =*/16).int32(message.type);
            return writer;
        };

        /**
         * Encodes the specified ApplicationFormQuestion message, length delimited. Does not implicitly {@link OpeningMetadata.ApplicationFormQuestion.verify|verify} messages.
         * @function encodeDelimited
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {OpeningMetadata.IApplicationFormQuestion} message ApplicationFormQuestion message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ApplicationFormQuestion.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an ApplicationFormQuestion message from the specified reader or buffer.
         * @function decode
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {OpeningMetadata.ApplicationFormQuestion} ApplicationFormQuestion
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ApplicationFormQuestion.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.OpeningMetadata.ApplicationFormQuestion();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.question = reader.string();
                    break;
                case 2:
                    message.type = reader.int32();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an ApplicationFormQuestion message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {OpeningMetadata.ApplicationFormQuestion} ApplicationFormQuestion
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ApplicationFormQuestion.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an ApplicationFormQuestion message.
         * @function verify
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        ApplicationFormQuestion.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.question != null && message.hasOwnProperty("question"))
                if (!$util.isString(message.question))
                    return "question: string expected";
            if (message.type != null && message.hasOwnProperty("type"))
                switch (message.type) {
                default:
                    return "type: enum value expected";
                case 0:
                case 1:
                    break;
                }
            return null;
        };

        /**
         * Creates an ApplicationFormQuestion message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {OpeningMetadata.ApplicationFormQuestion} ApplicationFormQuestion
         */
        ApplicationFormQuestion.fromObject = function fromObject(object) {
            if (object instanceof $root.OpeningMetadata.ApplicationFormQuestion)
                return object;
            var message = new $root.OpeningMetadata.ApplicationFormQuestion();
            if (object.question != null)
                message.question = String(object.question);
            switch (object.type) {
            case "TEXTAREA":
            case 0:
                message.type = 0;
                break;
            case "TEXT":
            case 1:
                message.type = 1;
                break;
            }
            return message;
        };

        /**
         * Creates a plain object from an ApplicationFormQuestion message. Also converts values to other types if specified.
         * @function toObject
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @static
         * @param {OpeningMetadata.ApplicationFormQuestion} message ApplicationFormQuestion
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        ApplicationFormQuestion.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.question = "";
                object.type = options.enums === String ? "TEXTAREA" : 0;
            }
            if (message.question != null && message.hasOwnProperty("question"))
                object.question = message.question;
            if (message.type != null && message.hasOwnProperty("type"))
                object.type = options.enums === String ? $root.OpeningMetadata.ApplicationFormQuestion.InputType[message.type] : message.type;
            return object;
        };

        /**
         * Converts this ApplicationFormQuestion to JSON.
         * @function toJSON
         * @memberof OpeningMetadata.ApplicationFormQuestion
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        ApplicationFormQuestion.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * InputType enum.
         * @name OpeningMetadata.ApplicationFormQuestion.InputType
         * @enum {number}
         * @property {number} TEXTAREA=0 TEXTAREA value
         * @property {number} TEXT=1 TEXT value
         */
        ApplicationFormQuestion.InputType = (function() {
            var valuesById = {}, values = Object.create(valuesById);
            values[valuesById[0] = "TEXTAREA"] = 0;
            values[valuesById[1] = "TEXT"] = 1;
            return values;
        })();

        return ApplicationFormQuestion;
    })();

    return OpeningMetadata;
})();

$root.UpcomingOpeningMetadata = (function() {

    /**
     * Properties of an UpcomingOpeningMetadata.
     * @exports IUpcomingOpeningMetadata
     * @interface IUpcomingOpeningMetadata
     * @property {number|null} [expectedStart] UpcomingOpeningMetadata expectedStart
     * @property {Long|null} [rewardPerBlock] UpcomingOpeningMetadata rewardPerBlock
     * @property {Long|null} [minApplicationStake] UpcomingOpeningMetadata minApplicationStake
     * @property {IOpeningMetadata|null} [metadata] UpcomingOpeningMetadata metadata
     */

    /**
     * Constructs a new UpcomingOpeningMetadata.
     * @exports UpcomingOpeningMetadata
     * @classdesc Represents an UpcomingOpeningMetadata.
     * @implements IUpcomingOpeningMetadata
     * @constructor
     * @param {IUpcomingOpeningMetadata=} [properties] Properties to set
     */
    function UpcomingOpeningMetadata(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * UpcomingOpeningMetadata expectedStart.
     * @member {number} expectedStart
     * @memberof UpcomingOpeningMetadata
     * @instance
     */
    UpcomingOpeningMetadata.prototype.expectedStart = 0;

    /**
     * UpcomingOpeningMetadata rewardPerBlock.
     * @member {Long} rewardPerBlock
     * @memberof UpcomingOpeningMetadata
     * @instance
     */
    UpcomingOpeningMetadata.prototype.rewardPerBlock = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

    /**
     * UpcomingOpeningMetadata minApplicationStake.
     * @member {Long} minApplicationStake
     * @memberof UpcomingOpeningMetadata
     * @instance
     */
    UpcomingOpeningMetadata.prototype.minApplicationStake = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

    /**
     * UpcomingOpeningMetadata metadata.
     * @member {IOpeningMetadata|null|undefined} metadata
     * @memberof UpcomingOpeningMetadata
     * @instance
     */
    UpcomingOpeningMetadata.prototype.metadata = null;

    /**
     * Creates a new UpcomingOpeningMetadata instance using the specified properties.
     * @function create
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {IUpcomingOpeningMetadata=} [properties] Properties to set
     * @returns {UpcomingOpeningMetadata} UpcomingOpeningMetadata instance
     */
    UpcomingOpeningMetadata.create = function create(properties) {
        return new UpcomingOpeningMetadata(properties);
    };

    /**
     * Encodes the specified UpcomingOpeningMetadata message. Does not implicitly {@link UpcomingOpeningMetadata.verify|verify} messages.
     * @function encode
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {IUpcomingOpeningMetadata} message UpcomingOpeningMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    UpcomingOpeningMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.expectedStart != null && Object.hasOwnProperty.call(message, "expectedStart"))
            writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.expectedStart);
        if (message.rewardPerBlock != null && Object.hasOwnProperty.call(message, "rewardPerBlock"))
            writer.uint32(/* id 2, wireType 0 =*/16).uint64(message.rewardPerBlock);
        if (message.minApplicationStake != null && Object.hasOwnProperty.call(message, "minApplicationStake"))
            writer.uint32(/* id 3, wireType 0 =*/24).uint64(message.minApplicationStake);
        if (message.metadata != null && Object.hasOwnProperty.call(message, "metadata"))
            $root.OpeningMetadata.encode(message.metadata, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified UpcomingOpeningMetadata message, length delimited. Does not implicitly {@link UpcomingOpeningMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {IUpcomingOpeningMetadata} message UpcomingOpeningMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    UpcomingOpeningMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes an UpcomingOpeningMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {UpcomingOpeningMetadata} UpcomingOpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    UpcomingOpeningMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.UpcomingOpeningMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.expectedStart = reader.uint32();
                break;
            case 2:
                message.rewardPerBlock = reader.uint64();
                break;
            case 3:
                message.minApplicationStake = reader.uint64();
                break;
            case 4:
                message.metadata = $root.OpeningMetadata.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes an UpcomingOpeningMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {UpcomingOpeningMetadata} UpcomingOpeningMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    UpcomingOpeningMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies an UpcomingOpeningMetadata message.
     * @function verify
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    UpcomingOpeningMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.expectedStart != null && message.hasOwnProperty("expectedStart"))
            if (!$util.isInteger(message.expectedStart))
                return "expectedStart: integer expected";
        if (message.rewardPerBlock != null && message.hasOwnProperty("rewardPerBlock"))
            if (!$util.isInteger(message.rewardPerBlock) && !(message.rewardPerBlock && $util.isInteger(message.rewardPerBlock.low) && $util.isInteger(message.rewardPerBlock.high)))
                return "rewardPerBlock: integer|Long expected";
        if (message.minApplicationStake != null && message.hasOwnProperty("minApplicationStake"))
            if (!$util.isInteger(message.minApplicationStake) && !(message.minApplicationStake && $util.isInteger(message.minApplicationStake.low) && $util.isInteger(message.minApplicationStake.high)))
                return "minApplicationStake: integer|Long expected";
        if (message.metadata != null && message.hasOwnProperty("metadata")) {
            var error = $root.OpeningMetadata.verify(message.metadata);
            if (error)
                return "metadata." + error;
        }
        return null;
    };

    /**
     * Creates an UpcomingOpeningMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {UpcomingOpeningMetadata} UpcomingOpeningMetadata
     */
    UpcomingOpeningMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.UpcomingOpeningMetadata)
            return object;
        var message = new $root.UpcomingOpeningMetadata();
        if (object.expectedStart != null)
            message.expectedStart = object.expectedStart >>> 0;
        if (object.rewardPerBlock != null)
            if ($util.Long)
                (message.rewardPerBlock = $util.Long.fromValue(object.rewardPerBlock)).unsigned = true;
            else if (typeof object.rewardPerBlock === "string")
                message.rewardPerBlock = parseInt(object.rewardPerBlock, 10);
            else if (typeof object.rewardPerBlock === "number")
                message.rewardPerBlock = object.rewardPerBlock;
            else if (typeof object.rewardPerBlock === "object")
                message.rewardPerBlock = new $util.LongBits(object.rewardPerBlock.low >>> 0, object.rewardPerBlock.high >>> 0).toNumber(true);
        if (object.minApplicationStake != null)
            if ($util.Long)
                (message.minApplicationStake = $util.Long.fromValue(object.minApplicationStake)).unsigned = true;
            else if (typeof object.minApplicationStake === "string")
                message.minApplicationStake = parseInt(object.minApplicationStake, 10);
            else if (typeof object.minApplicationStake === "number")
                message.minApplicationStake = object.minApplicationStake;
            else if (typeof object.minApplicationStake === "object")
                message.minApplicationStake = new $util.LongBits(object.minApplicationStake.low >>> 0, object.minApplicationStake.high >>> 0).toNumber(true);
        if (object.metadata != null) {
            if (typeof object.metadata !== "object")
                throw TypeError(".UpcomingOpeningMetadata.metadata: object expected");
            message.metadata = $root.OpeningMetadata.fromObject(object.metadata);
        }
        return message;
    };

    /**
     * Creates a plain object from an UpcomingOpeningMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof UpcomingOpeningMetadata
     * @static
     * @param {UpcomingOpeningMetadata} message UpcomingOpeningMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    UpcomingOpeningMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.expectedStart = 0;
            if ($util.Long) {
                var long = new $util.Long(0, 0, true);
                object.rewardPerBlock = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.rewardPerBlock = options.longs === String ? "0" : 0;
            if ($util.Long) {
                var long = new $util.Long(0, 0, true);
                object.minApplicationStake = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.minApplicationStake = options.longs === String ? "0" : 0;
            object.metadata = null;
        }
        if (message.expectedStart != null && message.hasOwnProperty("expectedStart"))
            object.expectedStart = message.expectedStart;
        if (message.rewardPerBlock != null && message.hasOwnProperty("rewardPerBlock"))
            if (typeof message.rewardPerBlock === "number")
                object.rewardPerBlock = options.longs === String ? String(message.rewardPerBlock) : message.rewardPerBlock;
            else
                object.rewardPerBlock = options.longs === String ? $util.Long.prototype.toString.call(message.rewardPerBlock) : options.longs === Number ? new $util.LongBits(message.rewardPerBlock.low >>> 0, message.rewardPerBlock.high >>> 0).toNumber(true) : message.rewardPerBlock;
        if (message.minApplicationStake != null && message.hasOwnProperty("minApplicationStake"))
            if (typeof message.minApplicationStake === "number")
                object.minApplicationStake = options.longs === String ? String(message.minApplicationStake) : message.minApplicationStake;
            else
                object.minApplicationStake = options.longs === String ? $util.Long.prototype.toString.call(message.minApplicationStake) : options.longs === Number ? new $util.LongBits(message.minApplicationStake.low >>> 0, message.minApplicationStake.high >>> 0).toNumber(true) : message.minApplicationStake;
        if (message.metadata != null && message.hasOwnProperty("metadata"))
            object.metadata = $root.OpeningMetadata.toObject(message.metadata, options);
        return object;
    };

    /**
     * Converts this UpcomingOpeningMetadata to JSON.
     * @function toJSON
     * @memberof UpcomingOpeningMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    UpcomingOpeningMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return UpcomingOpeningMetadata;
})();

$root.ApplicationMetadata = (function() {

    /**
     * Properties of an ApplicationMetadata.
     * @exports IApplicationMetadata
     * @interface IApplicationMetadata
     * @property {Array.<string>|null} [answers] ApplicationMetadata answers
     */

    /**
     * Constructs a new ApplicationMetadata.
     * @exports ApplicationMetadata
     * @classdesc Represents an ApplicationMetadata.
     * @implements IApplicationMetadata
     * @constructor
     * @param {IApplicationMetadata=} [properties] Properties to set
     */
    function ApplicationMetadata(properties) {
        this.answers = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * ApplicationMetadata answers.
     * @member {Array.<string>} answers
     * @memberof ApplicationMetadata
     * @instance
     */
    ApplicationMetadata.prototype.answers = $util.emptyArray;

    /**
     * Creates a new ApplicationMetadata instance using the specified properties.
     * @function create
     * @memberof ApplicationMetadata
     * @static
     * @param {IApplicationMetadata=} [properties] Properties to set
     * @returns {ApplicationMetadata} ApplicationMetadata instance
     */
    ApplicationMetadata.create = function create(properties) {
        return new ApplicationMetadata(properties);
    };

    /**
     * Encodes the specified ApplicationMetadata message. Does not implicitly {@link ApplicationMetadata.verify|verify} messages.
     * @function encode
     * @memberof ApplicationMetadata
     * @static
     * @param {IApplicationMetadata} message ApplicationMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ApplicationMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.answers != null && message.answers.length)
            for (var i = 0; i < message.answers.length; ++i)
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.answers[i]);
        return writer;
    };

    /**
     * Encodes the specified ApplicationMetadata message, length delimited. Does not implicitly {@link ApplicationMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof ApplicationMetadata
     * @static
     * @param {IApplicationMetadata} message ApplicationMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ApplicationMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes an ApplicationMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof ApplicationMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {ApplicationMetadata} ApplicationMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ApplicationMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.ApplicationMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                if (!(message.answers && message.answers.length))
                    message.answers = [];
                message.answers.push(reader.string());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes an ApplicationMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof ApplicationMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {ApplicationMetadata} ApplicationMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ApplicationMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies an ApplicationMetadata message.
     * @function verify
     * @memberof ApplicationMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    ApplicationMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.answers != null && message.hasOwnProperty("answers")) {
            if (!Array.isArray(message.answers))
                return "answers: array expected";
            for (var i = 0; i < message.answers.length; ++i)
                if (!$util.isString(message.answers[i]))
                    return "answers: string[] expected";
        }
        return null;
    };

    /**
     * Creates an ApplicationMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof ApplicationMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {ApplicationMetadata} ApplicationMetadata
     */
    ApplicationMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.ApplicationMetadata)
            return object;
        var message = new $root.ApplicationMetadata();
        if (object.answers) {
            if (!Array.isArray(object.answers))
                throw TypeError(".ApplicationMetadata.answers: array expected");
            message.answers = [];
            for (var i = 0; i < object.answers.length; ++i)
                message.answers[i] = String(object.answers[i]);
        }
        return message;
    };

    /**
     * Creates a plain object from an ApplicationMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof ApplicationMetadata
     * @static
     * @param {ApplicationMetadata} message ApplicationMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    ApplicationMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.answers = [];
        if (message.answers && message.answers.length) {
            object.answers = [];
            for (var j = 0; j < message.answers.length; ++j)
                object.answers[j] = message.answers[j];
        }
        return object;
    };

    /**
     * Converts this ApplicationMetadata to JSON.
     * @function toJSON
     * @memberof ApplicationMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    ApplicationMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return ApplicationMetadata;
})();

$root.WorkingGroupMetadata = (function() {

    /**
     * Properties of a WorkingGroupMetadata.
     * @exports IWorkingGroupMetadata
     * @interface IWorkingGroupMetadata
     * @property {string|null} [description] WorkingGroupMetadata description
     * @property {string|null} [about] WorkingGroupMetadata about
     * @property {string|null} [status] WorkingGroupMetadata status
     * @property {string|null} [statusMessage] WorkingGroupMetadata statusMessage
     */

    /**
     * Constructs a new WorkingGroupMetadata.
     * @exports WorkingGroupMetadata
     * @classdesc Represents a WorkingGroupMetadata.
     * @implements IWorkingGroupMetadata
     * @constructor
     * @param {IWorkingGroupMetadata=} [properties] Properties to set
     */
    function WorkingGroupMetadata(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * WorkingGroupMetadata description.
     * @member {string} description
     * @memberof WorkingGroupMetadata
     * @instance
     */
    WorkingGroupMetadata.prototype.description = "";

    /**
     * WorkingGroupMetadata about.
     * @member {string} about
     * @memberof WorkingGroupMetadata
     * @instance
     */
    WorkingGroupMetadata.prototype.about = "";

    /**
     * WorkingGroupMetadata status.
     * @member {string} status
     * @memberof WorkingGroupMetadata
     * @instance
     */
    WorkingGroupMetadata.prototype.status = "";

    /**
     * WorkingGroupMetadata statusMessage.
     * @member {string} statusMessage
     * @memberof WorkingGroupMetadata
     * @instance
     */
    WorkingGroupMetadata.prototype.statusMessage = "";

    /**
     * Creates a new WorkingGroupMetadata instance using the specified properties.
     * @function create
     * @memberof WorkingGroupMetadata
     * @static
     * @param {IWorkingGroupMetadata=} [properties] Properties to set
     * @returns {WorkingGroupMetadata} WorkingGroupMetadata instance
     */
    WorkingGroupMetadata.create = function create(properties) {
        return new WorkingGroupMetadata(properties);
    };

    /**
     * Encodes the specified WorkingGroupMetadata message. Does not implicitly {@link WorkingGroupMetadata.verify|verify} messages.
     * @function encode
     * @memberof WorkingGroupMetadata
     * @static
     * @param {IWorkingGroupMetadata} message WorkingGroupMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    WorkingGroupMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.description != null && Object.hasOwnProperty.call(message, "description"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.description);
        if (message.about != null && Object.hasOwnProperty.call(message, "about"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.about);
        if (message.status != null && Object.hasOwnProperty.call(message, "status"))
            writer.uint32(/* id 3, wireType 2 =*/26).string(message.status);
        if (message.statusMessage != null && Object.hasOwnProperty.call(message, "statusMessage"))
            writer.uint32(/* id 4, wireType 2 =*/34).string(message.statusMessage);
        return writer;
    };

    /**
     * Encodes the specified WorkingGroupMetadata message, length delimited. Does not implicitly {@link WorkingGroupMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof WorkingGroupMetadata
     * @static
     * @param {IWorkingGroupMetadata} message WorkingGroupMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    WorkingGroupMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a WorkingGroupMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof WorkingGroupMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {WorkingGroupMetadata} WorkingGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    WorkingGroupMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.WorkingGroupMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.description = reader.string();
                break;
            case 2:
                message.about = reader.string();
                break;
            case 3:
                message.status = reader.string();
                break;
            case 4:
                message.statusMessage = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a WorkingGroupMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof WorkingGroupMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {WorkingGroupMetadata} WorkingGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    WorkingGroupMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a WorkingGroupMetadata message.
     * @function verify
     * @memberof WorkingGroupMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    WorkingGroupMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.description != null && message.hasOwnProperty("description"))
            if (!$util.isString(message.description))
                return "description: string expected";
        if (message.about != null && message.hasOwnProperty("about"))
            if (!$util.isString(message.about))
                return "about: string expected";
        if (message.status != null && message.hasOwnProperty("status"))
            if (!$util.isString(message.status))
                return "status: string expected";
        if (message.statusMessage != null && message.hasOwnProperty("statusMessage"))
            if (!$util.isString(message.statusMessage))
                return "statusMessage: string expected";
        return null;
    };

    /**
     * Creates a WorkingGroupMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof WorkingGroupMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {WorkingGroupMetadata} WorkingGroupMetadata
     */
    WorkingGroupMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.WorkingGroupMetadata)
            return object;
        var message = new $root.WorkingGroupMetadata();
        if (object.description != null)
            message.description = String(object.description);
        if (object.about != null)
            message.about = String(object.about);
        if (object.status != null)
            message.status = String(object.status);
        if (object.statusMessage != null)
            message.statusMessage = String(object.statusMessage);
        return message;
    };

    /**
     * Creates a plain object from a WorkingGroupMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof WorkingGroupMetadata
     * @static
     * @param {WorkingGroupMetadata} message WorkingGroupMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    WorkingGroupMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.description = "";
            object.about = "";
            object.status = "";
            object.statusMessage = "";
        }
        if (message.description != null && message.hasOwnProperty("description"))
            object.description = message.description;
        if (message.about != null && message.hasOwnProperty("about"))
            object.about = message.about;
        if (message.status != null && message.hasOwnProperty("status"))
            object.status = message.status;
        if (message.statusMessage != null && message.hasOwnProperty("statusMessage"))
            object.statusMessage = message.statusMessage;
        return object;
    };

    /**
     * Converts this WorkingGroupMetadata to JSON.
     * @function toJSON
     * @memberof WorkingGroupMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    WorkingGroupMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return WorkingGroupMetadata;
})();

$root.SetGroupMetadata = (function() {

    /**
     * Properties of a SetGroupMetadata.
     * @exports ISetGroupMetadata
     * @interface ISetGroupMetadata
     * @property {IWorkingGroupMetadata|null} [newMetadata] SetGroupMetadata newMetadata
     */

    /**
     * Constructs a new SetGroupMetadata.
     * @exports SetGroupMetadata
     * @classdesc Represents a SetGroupMetadata.
     * @implements ISetGroupMetadata
     * @constructor
     * @param {ISetGroupMetadata=} [properties] Properties to set
     */
    function SetGroupMetadata(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * SetGroupMetadata newMetadata.
     * @member {IWorkingGroupMetadata|null|undefined} newMetadata
     * @memberof SetGroupMetadata
     * @instance
     */
    SetGroupMetadata.prototype.newMetadata = null;

    /**
     * Creates a new SetGroupMetadata instance using the specified properties.
     * @function create
     * @memberof SetGroupMetadata
     * @static
     * @param {ISetGroupMetadata=} [properties] Properties to set
     * @returns {SetGroupMetadata} SetGroupMetadata instance
     */
    SetGroupMetadata.create = function create(properties) {
        return new SetGroupMetadata(properties);
    };

    /**
     * Encodes the specified SetGroupMetadata message. Does not implicitly {@link SetGroupMetadata.verify|verify} messages.
     * @function encode
     * @memberof SetGroupMetadata
     * @static
     * @param {ISetGroupMetadata} message SetGroupMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    SetGroupMetadata.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.newMetadata != null && Object.hasOwnProperty.call(message, "newMetadata"))
            $root.WorkingGroupMetadata.encode(message.newMetadata, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified SetGroupMetadata message, length delimited. Does not implicitly {@link SetGroupMetadata.verify|verify} messages.
     * @function encodeDelimited
     * @memberof SetGroupMetadata
     * @static
     * @param {ISetGroupMetadata} message SetGroupMetadata message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    SetGroupMetadata.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a SetGroupMetadata message from the specified reader or buffer.
     * @function decode
     * @memberof SetGroupMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {SetGroupMetadata} SetGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    SetGroupMetadata.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.SetGroupMetadata();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.newMetadata = $root.WorkingGroupMetadata.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a SetGroupMetadata message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof SetGroupMetadata
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {SetGroupMetadata} SetGroupMetadata
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    SetGroupMetadata.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a SetGroupMetadata message.
     * @function verify
     * @memberof SetGroupMetadata
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    SetGroupMetadata.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.newMetadata != null && message.hasOwnProperty("newMetadata")) {
            var error = $root.WorkingGroupMetadata.verify(message.newMetadata);
            if (error)
                return "newMetadata." + error;
        }
        return null;
    };

    /**
     * Creates a SetGroupMetadata message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof SetGroupMetadata
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {SetGroupMetadata} SetGroupMetadata
     */
    SetGroupMetadata.fromObject = function fromObject(object) {
        if (object instanceof $root.SetGroupMetadata)
            return object;
        var message = new $root.SetGroupMetadata();
        if (object.newMetadata != null) {
            if (typeof object.newMetadata !== "object")
                throw TypeError(".SetGroupMetadata.newMetadata: object expected");
            message.newMetadata = $root.WorkingGroupMetadata.fromObject(object.newMetadata);
        }
        return message;
    };

    /**
     * Creates a plain object from a SetGroupMetadata message. Also converts values to other types if specified.
     * @function toObject
     * @memberof SetGroupMetadata
     * @static
     * @param {SetGroupMetadata} message SetGroupMetadata
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    SetGroupMetadata.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults)
            object.newMetadata = null;
        if (message.newMetadata != null && message.hasOwnProperty("newMetadata"))
            object.newMetadata = $root.WorkingGroupMetadata.toObject(message.newMetadata, options);
        return object;
    };

    /**
     * Converts this SetGroupMetadata to JSON.
     * @function toJSON
     * @memberof SetGroupMetadata
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    SetGroupMetadata.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return SetGroupMetadata;
})();

$root.AddUpcomingOpening = (function() {

    /**
     * Properties of an AddUpcomingOpening.
     * @exports IAddUpcomingOpening
     * @interface IAddUpcomingOpening
     * @property {IUpcomingOpeningMetadata|null} [metadata] AddUpcomingOpening metadata
     */

    /**
     * Constructs a new AddUpcomingOpening.
     * @exports AddUpcomingOpening
     * @classdesc Represents an AddUpcomingOpening.
     * @implements IAddUpcomingOpening
     * @constructor
     * @param {IAddUpcomingOpening=} [properties] Properties to set
     */
    function AddUpcomingOpening(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * AddUpcomingOpening metadata.
     * @member {IUpcomingOpeningMetadata|null|undefined} metadata
     * @memberof AddUpcomingOpening
     * @instance
     */
    AddUpcomingOpening.prototype.metadata = null;

    /**
     * Creates a new AddUpcomingOpening instance using the specified properties.
     * @function create
     * @memberof AddUpcomingOpening
     * @static
     * @param {IAddUpcomingOpening=} [properties] Properties to set
     * @returns {AddUpcomingOpening} AddUpcomingOpening instance
     */
    AddUpcomingOpening.create = function create(properties) {
        return new AddUpcomingOpening(properties);
    };

    /**
     * Encodes the specified AddUpcomingOpening message. Does not implicitly {@link AddUpcomingOpening.verify|verify} messages.
     * @function encode
     * @memberof AddUpcomingOpening
     * @static
     * @param {IAddUpcomingOpening} message AddUpcomingOpening message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    AddUpcomingOpening.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.metadata != null && Object.hasOwnProperty.call(message, "metadata"))
            $root.UpcomingOpeningMetadata.encode(message.metadata, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified AddUpcomingOpening message, length delimited. Does not implicitly {@link AddUpcomingOpening.verify|verify} messages.
     * @function encodeDelimited
     * @memberof AddUpcomingOpening
     * @static
     * @param {IAddUpcomingOpening} message AddUpcomingOpening message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    AddUpcomingOpening.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes an AddUpcomingOpening message from the specified reader or buffer.
     * @function decode
     * @memberof AddUpcomingOpening
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {AddUpcomingOpening} AddUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    AddUpcomingOpening.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.AddUpcomingOpening();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.metadata = $root.UpcomingOpeningMetadata.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes an AddUpcomingOpening message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof AddUpcomingOpening
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {AddUpcomingOpening} AddUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    AddUpcomingOpening.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies an AddUpcomingOpening message.
     * @function verify
     * @memberof AddUpcomingOpening
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    AddUpcomingOpening.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.metadata != null && message.hasOwnProperty("metadata")) {
            var error = $root.UpcomingOpeningMetadata.verify(message.metadata);
            if (error)
                return "metadata." + error;
        }
        return null;
    };

    /**
     * Creates an AddUpcomingOpening message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof AddUpcomingOpening
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {AddUpcomingOpening} AddUpcomingOpening
     */
    AddUpcomingOpening.fromObject = function fromObject(object) {
        if (object instanceof $root.AddUpcomingOpening)
            return object;
        var message = new $root.AddUpcomingOpening();
        if (object.metadata != null) {
            if (typeof object.metadata !== "object")
                throw TypeError(".AddUpcomingOpening.metadata: object expected");
            message.metadata = $root.UpcomingOpeningMetadata.fromObject(object.metadata);
        }
        return message;
    };

    /**
     * Creates a plain object from an AddUpcomingOpening message. Also converts values to other types if specified.
     * @function toObject
     * @memberof AddUpcomingOpening
     * @static
     * @param {AddUpcomingOpening} message AddUpcomingOpening
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    AddUpcomingOpening.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults)
            object.metadata = null;
        if (message.metadata != null && message.hasOwnProperty("metadata"))
            object.metadata = $root.UpcomingOpeningMetadata.toObject(message.metadata, options);
        return object;
    };

    /**
     * Converts this AddUpcomingOpening to JSON.
     * @function toJSON
     * @memberof AddUpcomingOpening
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    AddUpcomingOpening.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return AddUpcomingOpening;
})();

$root.RemoveUpcomingOpening = (function() {

    /**
     * Properties of a RemoveUpcomingOpening.
     * @exports IRemoveUpcomingOpening
     * @interface IRemoveUpcomingOpening
     * @property {string|null} [id] RemoveUpcomingOpening id
     */

    /**
     * Constructs a new RemoveUpcomingOpening.
     * @exports RemoveUpcomingOpening
     * @classdesc Represents a RemoveUpcomingOpening.
     * @implements IRemoveUpcomingOpening
     * @constructor
     * @param {IRemoveUpcomingOpening=} [properties] Properties to set
     */
    function RemoveUpcomingOpening(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * RemoveUpcomingOpening id.
     * @member {string} id
     * @memberof RemoveUpcomingOpening
     * @instance
     */
    RemoveUpcomingOpening.prototype.id = "";

    /**
     * Creates a new RemoveUpcomingOpening instance using the specified properties.
     * @function create
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {IRemoveUpcomingOpening=} [properties] Properties to set
     * @returns {RemoveUpcomingOpening} RemoveUpcomingOpening instance
     */
    RemoveUpcomingOpening.create = function create(properties) {
        return new RemoveUpcomingOpening(properties);
    };

    /**
     * Encodes the specified RemoveUpcomingOpening message. Does not implicitly {@link RemoveUpcomingOpening.verify|verify} messages.
     * @function encode
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {IRemoveUpcomingOpening} message RemoveUpcomingOpening message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    RemoveUpcomingOpening.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.id != null && Object.hasOwnProperty.call(message, "id"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
        return writer;
    };

    /**
     * Encodes the specified RemoveUpcomingOpening message, length delimited. Does not implicitly {@link RemoveUpcomingOpening.verify|verify} messages.
     * @function encodeDelimited
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {IRemoveUpcomingOpening} message RemoveUpcomingOpening message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    RemoveUpcomingOpening.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a RemoveUpcomingOpening message from the specified reader or buffer.
     * @function decode
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {RemoveUpcomingOpening} RemoveUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    RemoveUpcomingOpening.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.RemoveUpcomingOpening();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.id = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a RemoveUpcomingOpening message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {RemoveUpcomingOpening} RemoveUpcomingOpening
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    RemoveUpcomingOpening.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a RemoveUpcomingOpening message.
     * @function verify
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    RemoveUpcomingOpening.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.id != null && message.hasOwnProperty("id"))
            if (!$util.isString(message.id))
                return "id: string expected";
        return null;
    };

    /**
     * Creates a RemoveUpcomingOpening message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {RemoveUpcomingOpening} RemoveUpcomingOpening
     */
    RemoveUpcomingOpening.fromObject = function fromObject(object) {
        if (object instanceof $root.RemoveUpcomingOpening)
            return object;
        var message = new $root.RemoveUpcomingOpening();
        if (object.id != null)
            message.id = String(object.id);
        return message;
    };

    /**
     * Creates a plain object from a RemoveUpcomingOpening message. Also converts values to other types if specified.
     * @function toObject
     * @memberof RemoveUpcomingOpening
     * @static
     * @param {RemoveUpcomingOpening} message RemoveUpcomingOpening
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    RemoveUpcomingOpening.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults)
            object.id = "";
        if (message.id != null && message.hasOwnProperty("id"))
            object.id = message.id;
        return object;
    };

    /**
     * Converts this RemoveUpcomingOpening to JSON.
     * @function toJSON
     * @memberof RemoveUpcomingOpening
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    RemoveUpcomingOpening.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return RemoveUpcomingOpening;
})();

$root.WorkingGroupMetadataAction = (function() {

    /**
     * Properties of a WorkingGroupMetadataAction.
     * @exports IWorkingGroupMetadataAction
     * @interface IWorkingGroupMetadataAction
     * @property {ISetGroupMetadata|null} [setGroupMetadata] WorkingGroupMetadataAction setGroupMetadata
     * @property {IAddUpcomingOpening|null} [addUpcomingOpening] WorkingGroupMetadataAction addUpcomingOpening
     * @property {IRemoveUpcomingOpening|null} [removeUpcomingOpening] WorkingGroupMetadataAction removeUpcomingOpening
     */

    /**
     * Constructs a new WorkingGroupMetadataAction.
     * @exports WorkingGroupMetadataAction
     * @classdesc Represents a WorkingGroupMetadataAction.
     * @implements IWorkingGroupMetadataAction
     * @constructor
     * @param {IWorkingGroupMetadataAction=} [properties] Properties to set
     */
    function WorkingGroupMetadataAction(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * WorkingGroupMetadataAction setGroupMetadata.
     * @member {ISetGroupMetadata|null|undefined} setGroupMetadata
     * @memberof WorkingGroupMetadataAction
     * @instance
     */
    WorkingGroupMetadataAction.prototype.setGroupMetadata = null;

    /**
     * WorkingGroupMetadataAction addUpcomingOpening.
     * @member {IAddUpcomingOpening|null|undefined} addUpcomingOpening
     * @memberof WorkingGroupMetadataAction
     * @instance
     */
    WorkingGroupMetadataAction.prototype.addUpcomingOpening = null;

    /**
     * WorkingGroupMetadataAction removeUpcomingOpening.
     * @member {IRemoveUpcomingOpening|null|undefined} removeUpcomingOpening
     * @memberof WorkingGroupMetadataAction
     * @instance
     */
    WorkingGroupMetadataAction.prototype.removeUpcomingOpening = null;

    // OneOf field names bound to virtual getters and setters
    var $oneOfFields;

    /**
     * WorkingGroupMetadataAction action.
     * @member {"setGroupMetadata"|"addUpcomingOpening"|"removeUpcomingOpening"|undefined} action
     * @memberof WorkingGroupMetadataAction
     * @instance
     */
    Object.defineProperty(WorkingGroupMetadataAction.prototype, "action", {
        get: $util.oneOfGetter($oneOfFields = ["setGroupMetadata", "addUpcomingOpening", "removeUpcomingOpening"]),
        set: $util.oneOfSetter($oneOfFields)
    });

    /**
     * Creates a new WorkingGroupMetadataAction instance using the specified properties.
     * @function create
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {IWorkingGroupMetadataAction=} [properties] Properties to set
     * @returns {WorkingGroupMetadataAction} WorkingGroupMetadataAction instance
     */
    WorkingGroupMetadataAction.create = function create(properties) {
        return new WorkingGroupMetadataAction(properties);
    };

    /**
     * Encodes the specified WorkingGroupMetadataAction message. Does not implicitly {@link WorkingGroupMetadataAction.verify|verify} messages.
     * @function encode
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {IWorkingGroupMetadataAction} message WorkingGroupMetadataAction message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    WorkingGroupMetadataAction.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.setGroupMetadata != null && Object.hasOwnProperty.call(message, "setGroupMetadata"))
            $root.SetGroupMetadata.encode(message.setGroupMetadata, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
        if (message.addUpcomingOpening != null && Object.hasOwnProperty.call(message, "addUpcomingOpening"))
            $root.AddUpcomingOpening.encode(message.addUpcomingOpening, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
        if (message.removeUpcomingOpening != null && Object.hasOwnProperty.call(message, "removeUpcomingOpening"))
            $root.RemoveUpcomingOpening.encode(message.removeUpcomingOpening, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified WorkingGroupMetadataAction message, length delimited. Does not implicitly {@link WorkingGroupMetadataAction.verify|verify} messages.
     * @function encodeDelimited
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {IWorkingGroupMetadataAction} message WorkingGroupMetadataAction message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    WorkingGroupMetadataAction.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a WorkingGroupMetadataAction message from the specified reader or buffer.
     * @function decode
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {WorkingGroupMetadataAction} WorkingGroupMetadataAction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    WorkingGroupMetadataAction.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.WorkingGroupMetadataAction();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.setGroupMetadata = $root.SetGroupMetadata.decode(reader, reader.uint32());
                break;
            case 2:
                message.addUpcomingOpening = $root.AddUpcomingOpening.decode(reader, reader.uint32());
                break;
            case 3:
                message.removeUpcomingOpening = $root.RemoveUpcomingOpening.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a WorkingGroupMetadataAction message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {WorkingGroupMetadataAction} WorkingGroupMetadataAction
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    WorkingGroupMetadataAction.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a WorkingGroupMetadataAction message.
     * @function verify
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    WorkingGroupMetadataAction.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        var properties = {};
        if (message.setGroupMetadata != null && message.hasOwnProperty("setGroupMetadata")) {
            properties.action = 1;
            {
                var error = $root.SetGroupMetadata.verify(message.setGroupMetadata);
                if (error)
                    return "setGroupMetadata." + error;
            }
        }
        if (message.addUpcomingOpening != null && message.hasOwnProperty("addUpcomingOpening")) {
            if (properties.action === 1)
                return "action: multiple values";
            properties.action = 1;
            {
                var error = $root.AddUpcomingOpening.verify(message.addUpcomingOpening);
                if (error)
                    return "addUpcomingOpening." + error;
            }
        }
        if (message.removeUpcomingOpening != null && message.hasOwnProperty("removeUpcomingOpening")) {
            if (properties.action === 1)
                return "action: multiple values";
            properties.action = 1;
            {
                var error = $root.RemoveUpcomingOpening.verify(message.removeUpcomingOpening);
                if (error)
                    return "removeUpcomingOpening." + error;
            }
        }
        return null;
    };

    /**
     * Creates a WorkingGroupMetadataAction message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {WorkingGroupMetadataAction} WorkingGroupMetadataAction
     */
    WorkingGroupMetadataAction.fromObject = function fromObject(object) {
        if (object instanceof $root.WorkingGroupMetadataAction)
            return object;
        var message = new $root.WorkingGroupMetadataAction();
        if (object.setGroupMetadata != null) {
            if (typeof object.setGroupMetadata !== "object")
                throw TypeError(".WorkingGroupMetadataAction.setGroupMetadata: object expected");
            message.setGroupMetadata = $root.SetGroupMetadata.fromObject(object.setGroupMetadata);
        }
        if (object.addUpcomingOpening != null) {
            if (typeof object.addUpcomingOpening !== "object")
                throw TypeError(".WorkingGroupMetadataAction.addUpcomingOpening: object expected");
            message.addUpcomingOpening = $root.AddUpcomingOpening.fromObject(object.addUpcomingOpening);
        }
        if (object.removeUpcomingOpening != null) {
            if (typeof object.removeUpcomingOpening !== "object")
                throw TypeError(".WorkingGroupMetadataAction.removeUpcomingOpening: object expected");
            message.removeUpcomingOpening = $root.RemoveUpcomingOpening.fromObject(object.removeUpcomingOpening);
        }
        return message;
    };

    /**
     * Creates a plain object from a WorkingGroupMetadataAction message. Also converts values to other types if specified.
     * @function toObject
     * @memberof WorkingGroupMetadataAction
     * @static
     * @param {WorkingGroupMetadataAction} message WorkingGroupMetadataAction
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    WorkingGroupMetadataAction.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (message.setGroupMetadata != null && message.hasOwnProperty("setGroupMetadata")) {
            object.setGroupMetadata = $root.SetGroupMetadata.toObject(message.setGroupMetadata, options);
            if (options.oneofs)
                object.action = "setGroupMetadata";
        }
        if (message.addUpcomingOpening != null && message.hasOwnProperty("addUpcomingOpening")) {
            object.addUpcomingOpening = $root.AddUpcomingOpening.toObject(message.addUpcomingOpening, options);
            if (options.oneofs)
                object.action = "addUpcomingOpening";
        }
        if (message.removeUpcomingOpening != null && message.hasOwnProperty("removeUpcomingOpening")) {
            object.removeUpcomingOpening = $root.RemoveUpcomingOpening.toObject(message.removeUpcomingOpening, options);
            if (options.oneofs)
                object.action = "removeUpcomingOpening";
        }
        return object;
    };

    /**
     * Converts this WorkingGroupMetadataAction to JSON.
     * @function toJSON
     * @memberof WorkingGroupMetadataAction
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    WorkingGroupMetadataAction.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return WorkingGroupMetadataAction;
})();

module.exports = $root;
